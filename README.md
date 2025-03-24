# Project

Attempt to integrate OpenTelemetry with Jaeger. The goal is to understand how to instrument applications to collect and visualize distributed traces.

## Technologies Used

- **OpenTelemetry**: For instrumentation and data collection of tracing.
- **Jaeger**: For visualization of the collected traces.

## Description

This project demonstrates how to configure a Rust application to send tracing data to a collector and visualize it in Jaeger. The configuration includes defining resources such as service name, version, and deployment environment.

## How to Run

To run the application, you can use Docker Compose. Follow the steps below:

1. **Start the containers**: Use the following command to start the containers in the background:

   ```bash
   docker compose up -d
   ```

2. **Access Jaeger**: After starting the containers, you can access the Jaeger interface in your browser at `<ip>:16686`, replacing `<ip>` with the appropriate IP address of your environment.



