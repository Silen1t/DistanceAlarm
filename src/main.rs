#![no_std]
#![no_main]

use cyw43::PowerManagementMode;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use ultra_measure::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

struct BuzzerCommand {
    active: bool,
}

static BUZZER_CHANNEL: Channel<CriticalSectionRawMutex, BuzzerCommand, 1> = Channel::new();

#[embassy_executor::task]
async fn buzzer_task(mut buzzer: Output<'static>) {
    let mut is_active = false;
    let mut buzzer_state = false;
    
    loop {
        // Check for new commands without blocking
        if let Ok(cmd) = BUZZER_CHANNEL.try_receive() {
            is_active = cmd.active;
            if !is_active {
                buzzer.set_low();
                buzzer_state = false;
            }
        }
        
        // Handle buzzer toggling
        if is_active {
            buzzer_state = !buzzer_state;
            if buzzer_state {
                buzzer.set_high();
            } else {
                buzzer.set_low();
            }
            Timer::after(Duration::from_millis(100)).await; // Beep interval
        } else {
            // When not active, just yield briefly
            Timer::after(Duration::from_millis(10)).await;
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);

    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());

    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(PowerManagementMode::PowerSave)
        .await;

    // Init LEDs
    let mut red_led = Output::new(p.PIN_15, Level::Low);
    let mut green_led = Output::new(p.PIN_14, Level::Low);

    // Init Buzzer
    let buzzer = Output::new(p.PIN_13, Level::Low);
    unwrap!(spawner.spawn(buzzer_task(buzzer)));

    // Init Sensor
    let sensor_trigger = Output::new(p.PIN_17, Level::Low);
    let sensor_echo = Input::new(p.PIN_16, Pull::None);

    let mut distance_sensor = UltraMeasure::new(sensor_trigger, sensor_echo);
    let detect_distance: f32 = 100.0;

    let delay = Duration::from_millis(250);

    loop {
        if let Ok(hit_distance) = distance_sensor.measure_distance(detect_distance).await {
            if hit_distance < 10.0 {
                red_led.set_low();
                green_led.set_high();
                let _ = BUZZER_CHANNEL.try_send(BuzzerCommand { active: true });
            } else {
                red_led.set_high();
                green_led.set_low();
                let _ = BUZZER_CHANNEL.try_send(BuzzerCommand { active: false });
            }
        } else {
            red_led.set_high();
            green_led.set_low();
            let _ = BUZZER_CHANNEL.try_send(BuzzerCommand { active: false });
        }

        Timer::after(delay).await;
    }
}