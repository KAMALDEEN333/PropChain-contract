# PropChain Smart Contracts

> 🏠 **Decentralized Real Estate Infrastructure** | Rust-based smart contracts for blockchain-powered property transactions

PropChain Smart Contracts is a production-grade Rust-based smart contract system that enables the tokenization and trading of real estate assets through blockchain technology. Our contracts provide the core functionality needed to build decentralized real estate platforms, including property ownership, secure transfers, and escrow services.

Built with Rust and ink! for Substrate/Polkadot ecosystem, these smart contracts serve as the foundation for Web3 real estate applications, enabling developers to create platforms where physical properties can be represented as digital assets and traded seamlessly using cryptocurrency.

## 🚀 Features

### Core Capabilities
- **🏠 Asset Tokenization**: Transform physical real estate properties into tradable NFTs with legal compliance
- **💰 Secure Transfers**: Multi-signature property transfers with escrow protection
- **🔗 Property Registry**: On-chain property ownership registry with metadata storage
- **📊 Fractional Ownership**: Enable partial ownership and investment pooling
- **🔐 Access Control**: Role-based permissions for property owners, agents, and regulators
- **💾 On-chain Storage**: Decentralized storage for property documents and metadata

### Advanced Features
- **⛓️ Cross-Chain Compatibility**: Designed for Substrate/Polkadot ecosystem with EVM compatibility
- **📈 Property Valuation**: On-chain valuation oracle integration for real-time pricing
- **🔍 Property Discovery**: Efficient on-chain search and filtering capabilities
- **📱 Mobile Integration**: Lightweight contract interfaces for mobile dApps
- **🛡️ Security First**: Formal verification and comprehensive audit coverage

## 👥 Target Audience

This smart contract system is designed for:
- **Real Estate Tech Companies** building blockchain-based property platforms
- **Property Investment Firms** seeking fractional ownership solutions
- **Blockchain Developers** creating DeFi real estate applications on Substrate
- **Real Estate Agencies** modernizing their transaction infrastructure
- **FinTech Startups** integrating real estate into their crypto ecosystems

## 🛠️ Quick Start

### Prerequisites
Ensure you have the following installed:
- **Rust** 1.70+ (stable toolchain)
- **ink! CLI** for smart contract development
- **Substrate Node** for local testing
- **Git** version control

### Installation

```bash
# 1. Clone the repository
git clone https://github.com/MettaChain/PropChain-contract.git
cd PropChain-contract

# 2. Install Rust and ink!
curl https://sh.rustup.rs -sSf | sh
cargo install cargo-contract --locked

# 3. Build the contracts
cargo contract build

# 4. Run tests
cargo test

# 5. Deploy locally (optional)
cargo contract instantiate --constructor new --args default
```

The contracts will be compiled and ready for deployment to Substrate-based networks.

## 🚀 Development & Deployment

### Development Environment
```bash
cargo contract build        # Build contracts in debug mode
cargo contract test         # Run unit tests
cargo test                 # Run all tests including integration
```

### Production Deployment
```bash
cargo contract build --release  # Build optimized contracts
cargo contract instantiate       # Deploy to network
cargo contract call             # Execute contract methods
```

### Testing Suite
```bash
cargo test                      # Run all tests
cargo contract test             # Contract-specific tests
cargo test --release            # Run tests in release mode
```

### Contract Management
```bash
cargo contract build            # Compile contracts
cargo contract instantiate      # Deploy to testnet
cargo contract upload          # Deploy to mainnet
cargo contract info            # Get contract information
```

## 🌐 Network Configuration

### Supported Blockchains
- **Polkadot** (Mainnet, Westend Testnet)
- **Kusama** (Mainnet)
- **Substrate-based Parachains** (Custom networks)
- **Local Development** (Substrate Node)

### Environment Configuration
```env
# Network
NETWORK=westend
NODE_URL=ws://localhost:9944

# Contract
CONTRACT_ACCOUNT=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
SURI=//Alice

# Build
BUILD_MODE=debug
TARGET=wasm32-unknown-unknown
```

## 📚 Documentation & Resources

### Contract Documentation
- **[📖 Contract API](./docs/contracts.md)** - Complete contract interface documentation
- **[🔗 Integration Guide](./docs/integration.md)** - How to integrate with frontend applications
- **[🚀 Deployment Guide](./docs/deployment.md)** - Contract deployment best practices
- **[🏗️ Architecture](./docs/architecture.md)** - Contract design and technical architecture

### Repository Structure
```
PropChain-contract/
├── 📁 contracts/           # Main smart contract source code
│   ├── 📁 lib/            # Contract logic and implementations
│   ├── 📁 traits/         # Shared trait definitions
│   └── 📁 tests/          # Contract unit tests
├── 📁 scripts/            # Deployment and utility scripts
├── 📁 tests/              # Integration and E2E tests
├── 📁 docs/               # Comprehensive documentation
├── 📁 .github/            # CI/CD workflows and issue templates
├── � Cargo.toml          # Rust project configuration
└── � .ink/               # ink! configuration files
```

### Contributing
- **[🤝 Contributing Guide](./CONTRIBUTING.md)** - How to contribute effectively
- **[📋 Code of Conduct](./CODE_OF_CONDUCT.md)** - Community guidelines and standards
- **[🐛 Issue Templates](./.github/ISSUE_TEMPLATE/)** - Standardized issue reporting
- **[💡 Feature Requests](./.github/ISSUE_TEMPLATE/feature_request.md)** - Feature proposal template

### Additional Resources
- **[🌐 Frontend Application](https://github.com/MettaChain/PropChain-FrontEnd)** - Client-side React/Next.js application
- **[🔒 Security Audits](./audits/)** - Third-party security audit reports
- **[📊 Performance Metrics](./docs/performance.md)** - Benchmarks and optimization guides
- **[🎓 Tutorials](./docs/tutorials/)** - Step-by-step integration tutorials

## 🛠️ Technology Stack

### Smart Contract Development
- **🦀 Language**: Rust - Memory safety and performance
- **⚡ Framework**: ink! - Substrate smart contract framework
- **⛓️ Platform**: Substrate/Polkadot - Enterprise blockchain framework
- **🔗 WASM**: WebAssembly compilation for blockchain deployment

### Development Tools
- **🛠️ Build**: Cargo - Rust package manager and build system
- **🧪 Testing**: Built-in Rust testing framework + ink! testing
- **� Documentation**: rustdoc - Auto-generated documentation
- **🔄 CI/CD**: GitHub Actions - Automated testing and deployment

### Blockchain Infrastructure
- **⛓️ Networks**: Polkadot, Kusama, Substrate parachains
- **🔐 Wallets**: Polkadot.js, Substrate-native wallets
- **📊 Oracles**: Chainlink, Substrate price feeds
- **� Explorers**: Subscan, Polkadot.js explorer

### Security & Verification
- **� Security**: Formal verification with cargo-contract
- **🛡️ Auditing**: Comprehensive security audit process
- **📋 Standards**: ERC-721/1155 compatibility layers
- **🔍 Testing**: Property-based testing with proptest

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for complete details.

## 🤝 Support & Community

### Get Help
- **🐛 Report Issues**: [GitHub Issues](https://github.com/MettaChain/PropChain-contract/issues)
- **📧 Email Support**: contracts@propchain.io
- **📖 Documentation**: [docs.propchain.io](https://docs.propchain.io)
- **💬 Discord**: [PropChain Community](https://discord.gg/propchain)

### Contributing
We welcome contributions! Please read our [Contributing Guide](./CONTRIBUTING.md) to get started. 

**Quick contribution steps:**
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`cargo test`)
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

---

<div align="center">

**⭐ Star this repository if it helped you!**

Made with ❤️ by the PropChain Team

</div>
