# Uber Fleet Manager (Desktop)

A high-performance, cross-platform desktop suite designed to centralize fleet operations, financial tracking, and automated data processing for Uber partners.

---

### 🛠️ Project Status: On-going Development
> **Currently working on:**
> * **Dynamic Column Mapping**: Developing a UI-based column selector so users can map any Excel layout to the internal data structures.
> * **PDF Export Engine**: Implementing a Rust-based PDF generator to create professional monthly reports for drivers.
> * **Data Persistence**: Integrating a SQLite database to track historical fleet performance over time.

---

## 🎯 Objective
To eliminate manual spreadsheet management by providing a centralized dashboard that automates the extraction and processing of complex earnings and expense data. The core engine replaces error-prone manual entry with a high-performance **Rust-powered parser**, ensuring 100% data integrity for financial reporting.

## ✨ Core Features

### 1. Automated Toll & Expense Processing
A specialized module designed to handle national toll exports (e.g., Via Verde).
* **Multi-Format Ingestion**: Seamlessly handles `.xlsx`, `.xls`, and `.csv` using the `calamine` engine.
* **Intelligent Aggregation**: Groups individual toll entries into per-vehicle summaries using $O(1)$ HashMap lookups.
* **Date Normalization**: Converts Excel’s internal serial timestamps into standardized ISO formats.

### 2. Fleet Financial Analytics
* **Earnings Breakdown**: Consolidates weekly Uber partner statements into actionable insights.
* **Expense Categorization**: Automatically distinguishes between Parking, Tolls, and Maintenance.
* **Profitability Mapping**: Per-vehicle performance tracking to identify the most efficient assets in the fleet.

### 3. Native Performance
* **Memory Safety**: Built with Rust to ensure zero-crash performance even when processing thousands of rows of financial data.
* **Low Footprint**: Optimized binary size and RAM usage compared to traditional Electron-based apps.

## 🛠 Tech Stack
* **Frontend**: React, TypeScript, TailwindCSS.
* **Backend/Core**: [Rust](https://www.rust-lang.org/) (Tauri).
* **Data Serialization**: [Serde](https://serde.rs/) (High-speed JSON interop).
* **Excel Engine**: [Calamine](https://github.com/tauri-apps/calamine).

## 🏗 System Architecture

The application utilizes the **Tauri Framework**, which isolates the UI from the system backend for maximum security and performance.

### Data Extraction Pipeline:
1. **Request**: React frontend sends a file path to the Rust core via a secure command.
2. **Parsing**: Rust opens the workbook and maps columns to strictly typed Structs.
3. **Logic**: The `extract_data_from_range` function performs grouping and arithmetic.
4. **Response**: Data is serialized into JSON and returned to the UI for immediate rendering.

## ⚙️ Getting Started

### Prerequisites
* [Rust](https://www.rust-lang.org/tools/install) (latest stable)
* [Node.js](https://nodejs.org/) (v18+)

### Installation
1. **Clone the repository**
   ```bash
   git clone [https://github.com/yourusername/uber-fleet-manager.git](https://github.com/yourusername/uber-fleet-manager.git)
   cd uber-fleet-manager
2. **Install Frontend Dependencies**

   ```bash
   npm install
3. **Run in Development Mode**
   ```bash
   npm run tauri dev

## 📄 License
This project is licensed under the GNU General Public License v3.0 (GPL-3.0). This ensures that the software remains free and open-source, protecting the work from being used in proprietary or commercial closed-source products.
