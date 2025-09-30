# DistanceAlarm

**Real-time motion detection and proximity monitoring system built with Rust for Raspberry Pi Pico W using ultrasonic sensor technology.**

## Description

DistanceAlarm is an embedded security and monitoring application that leverages my custom UltraMeasure library to provide intelligent motion detection and object proximity sensing. The system continuously monitors a defined area and triggers alerts when movement is detected or objects enter the monitored zone, making it ideal for security applications, automation systems, and proximity-based controls.

Built with async Rust and the Embassy framework, the system delivers reliable, non-blocking performance perfect for real-time monitoring scenarios.

## 🔧 Key Features

### Core Functionality

- **Real-time Motion Detection** - Continuously monitors for movement within the sensor range
- **Object Proximity Sensing** - Detects when objects enter predefined distance thresholds
- **Configurable Alert System** - Customizable alarm triggers based on distance and movement patterns
- **Non-blocking Operation** - Async architecture ensures responsive monitoring without system delays

## 🛠️ Technologies Used

- **Rust** - Systems programming language for embedded development
- **Embassy** - Async embedded framework for real-time applications
- **UltraMeasure Library** - Custom ultrasonic sensor control library
- **Raspberry Pi Pico W** - RP2040-based microcontroller platform
- **GPIO/Hardware Interfacing** - Direct sensor

## 💡 Project Highlights

This project demonstrates the practical application of embedded Rust development, showcasing how custom libraries can be leveraged to build complete, production-ready solutions. The combination of async programming, real-time sensor processing, and configurable alert systems creates a versatile foundation for various monitoring and automation applications.