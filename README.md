# Netskope Tool (Rust)

Command-line tool written in Rust to automate common tasks against a Netskope tenant via REST API.

Current main goals:

- Create **Private Apps** in bulk from an Excel spreadsheet.

> ⚠️ This is a personal/lab project. It is **not** an official Netskope tool.

## Features


### 1. Create Private Apps from Excel

- Reads an `applications.xlsx` spreadsheet from the project directory.
- For each row in the **`Applications`** sheet, it builds the JSON body required by Netskope’s Private Apps API.
- Sends a `POST` request to create each Private App.


  
## Prerequisites 

    • Rust installed (stable toolchain).
    • A Netskope tenant with:
    • Tenant URL (e.g. https://customer.goskope.com);
    • Valid API token (Bearer token) with permissions to read publishers and create private apps.
    • Network access from your machine to the Netskope tenant.

## Pre-built Windows executable

For convenience, this repository also includes a pre-built Windows executable:

- File: `netskope_tool.exe`
- Location: **project root** (same folder as `Cargo.toml`)



