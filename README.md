# Arise

**Arise** is a lightweight observability platform powered by **eBPF**, designed for real-time monitoring and network defense.  
It provides visibility into system events like **command execution**, **network traffic**, and **IP-based firewall activity** — all with minimal overhead.

The platform features:
- A web-based frontend built with **Vue.js**
- A backend API service built with **Actix Web** (Rust)
- **SurrealDB** for flexible, high-performance storage.

---

## Features

- 🛡️ **Command Execution Monitoring**  
  Capture and observe commands executed on the system in real-time.

- 🌐 **Network Traffic Monitoring**  
  Track inbound and outbound network activity for greater insight into your environment.

- 🚫 **IP-based Firewall (Whitelist/Blacklist)**  
  Allow or block traffic dynamically based on IP address policies.

- 📊 **Web Dashboard**  
  Visualize events, network traffic, and firewall logs using an intuitive Vue.js frontend.

- ⚡ **High Performance**  
  Leveraging eBPF ensures observability with minimal performance impact.


  # SurrealDB

   surreal start --user root --password root --bind=0.0.0.0:4050 rocksdb:netstudio.db
