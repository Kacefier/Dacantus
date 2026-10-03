# Dacantus

A lightweight CLI tool that encodes files into audio and decodes them back.  

---

## English

### Introduction

Dacantus is a lightweight file-to-audio codec that encodes arbitrary files into PCM audio (WAV) and decodes them back losslessly.  
It uses 2-FSK modulation at 2400 baud on a 9600 Hz mono carrier, wraps the payload in a self-describing `DCT1` container, and protects every frame with CRC-32.  
Written in Rust, it is distributed as a single binary file with no external dependencies.  

### Features

- Encode any file into a `.wav` audio file
- Decode a `.wav` back into the original file, byte-for-byte
- Inspect a Dacantus `.wav` without writing any output
- Self-describing `DCT1` container with magic, version and file name
- Per-frame CRC-32 plus a whole-payload CRC-32
- Robust sync: preamble + `DCT1` magic lock, no magic means no lock
- Pure Rust, no external dependencies at runtime

### Quick Start

First, download the appropriate executable file and installation script from the [Releases](https://github.com/Kacefier/Dacantus/releases) page.  
Then run the installation script to install.  
During installation, the script will prompt you to enter the path to the executable file—simply provide the path to the downloaded executable.  

If you cannot find a suitable executable in Releases, try cloning the repository and building manually.

### Usage Examples

| Command | Description |
|---------|-------------|
| `dacantus -v` | Show version information |
| `dacantus -h` | Show help information |
| `dacantus -e file.txt` | Encode `file.txt` into `file.wav` |
| `dacantus -d file.wav` | Decode `file.wav` back into `file.txt` |
| `dacantus -i file.wav` | Show info about a Dacantus `.wav`, no output written |

### Protocol

| Parameter | Value |
|-----------|-------|
| Modulation | 2-FSK |
| Baud rate | 2400 baud |
| Sample rate | 9600 Hz mono |
| Samples per bit | 4 |
| Mark frequency (bit 0) | 1800 Hz |
| Space frequency (bit 1) | 3600 Hz |
| Preamble | 32 × `0xAA` |
| Container magic | `DCT1` |
| Protocol version | 1 |
| Frame magic | `FRM1` |
| End magic | `END1` |
| Max frame payload | 256 bytes |
| Max file name length | 255 bytes |
| Checksum | CRC-32 per frame + whole payload |

### Pre-compiled Platforms

Pre-compiled executables for the following platforms are available on the Releases page:  

- Windows (amd64)
- Linux (amd64)

If your operating system is not listed, please try building manually.  
Additionally, the repository provides installation and uninstallation scripts for both Windows and macOS/Linux platforms.  

### Open Source License

This program is free software, licensed under the GNU General Public License v3.0.  

### Author

Kacefier  

GitHub: https://github.com/Kacefier  
Email: kacefier@zohomail.com  

---

## 中文

### 简介

Dacantus 是一款轻量级的文件与音频互转工具，可以把任意文件编码成 PCM 音频（WAV），并无损地解码还原。  
它使用 2-FSK 调制，波特率 2400 baud，载波为 9600 Hz 单声道，载荷封装在自描述的 `DCT1` 容器中，并对每一帧使用 CRC-32 校验。  
使用 Rust 编写，以单文件二进制形式分发，无需外部依赖。  

### 功能

- 将任意文件编码为 `.wav` 音频文件
- 将 `.wav` 解码还原为原始文件，逐字节一致
- 查看 Dacantus `.wav` 的信息，不写出任何文件
- 自描述的 `DCT1` 容器，包含魔数、版本和文件名
- 每帧 CRC-32 加整体载荷 CRC-32 双重校验
- 稳健的同步：前导码 + `DCT1` 魔数锁定，无魔数即无锁定
- 纯 Rust 实现，运行时不依赖外部库

### 快速开始

请先到 [Releases](https://github.com/Kacefier/Dacantus/releases) 页面下载合适的可执行文件和安装脚本。  
然后运行安装脚本进行安装。  
安装时，安装脚本会提示输入可执行文件的路径，输入下载的可执行文件的路径即可。  

如果你没有在 Releases 里找到合适的可执行文件，请尝试克隆仓库后手动编译。  

### 使用示例

| 命令 | 说明 |
|------|------|
| `dacantus -v` | 显示版本信息 |
| `dacantus -h` | 显示帮助信息 |
| `dacantus -e 文件.txt` | 将 `文件.txt` 编码为 `文件.wav` |
| `dacantus -d 文件.wav` | 将 `文件.wav` 解码还原为 `文件.txt` |
| `dacantus -i 文件.wav` | 查看 Dacantus `.wav` 的信息，不写出文件 |

### 协议参数

| 参数 | 取值 |
|------|------|
| 调制方式 | 2-FSK |
| 波特率 | 2400 baud |
| 采样率 | 9600 Hz 单声道 |
| 每比特采样数 | 4 |
| Mark 频率（bit 0） | 1800 Hz |
| Space 频率（bit 1） | 3600 Hz |
| 前导码 | 32 × `0xAA` |
| 容器魔数 | `DCT1` |
| 协议版本 | 1 |
| 帧魔数 | `FRM1` |
| 结束魔数 | `END1` |
| 单帧最大载荷 | 256 字节 |
| 文件名最大长度 | 255 字节 |
| 校验方式 | 每帧 CRC-32 + 整体载荷 CRC-32 |

### 预编译平台

发布页提供以下平台的预编译可执行文件：  

- Windows（amd64）
- Linux（amd64）

如果没有你使用的操作系统，请尝试手动编译。  
同时，仓库也提供 Windows 和 macOS/Linux 平台的安装与卸载脚本。  

### 开源许可证

本程序是自由软件，采用 GNU 通用公共许可证 v3.0 授权。  

### 作者

Kacefier  

GitHub：https://github.com/Kacefier  
邮箱：kacefier@zohomail.com  
