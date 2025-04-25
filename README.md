# Upcycled Art Installation
![asd](https://github.com/user-attachments/assets/9959dce9-9d25-4b99-8262-09bfaf523f91)


# About the Project
Upcycled Art Installation is a blockchain-based platform that connects artists who create art from recycled materials with exhibition spaces, collectors, and the general public. This decentralized solution enables transparent registration of art pieces, verification of their authenticity, and maintenance of a ledger documenting upcycled materials used in each creation. The platform aims to promote sustainability in art by incentivizing the use of recycled materials while providing provenance tracking for each installation.

The smart contract, built on Soroban, facilitates registration of art pieces, verification by curators, and maintains platform statistics to track the environmental impact of upcycled art.

# Vision
Our vision is to revolutionize the intersection of art and sustainability through blockchain technology. By establishing a transparent ecosystem for upcycled art, we aim to reduce waste, empower artists, and transform how society views discarded materials. Upcycled Art Installation will create a global community where creativity and environmental consciousness converge, inspiring a new generation of sustainable artists and collectors. Through this platform, we will quantify and showcase the environmental impact of art, creating a tangible bridge between artistic expression and ecological responsibility.

# Features
Art Registration: Artists can register their upcycled creations with details about materials used
Verification System: Curators verify art pieces after confirming material authenticity
Art Piece Information: Users can retrieve detailed information about any registered art piece
Platform Statistics: Maintains statistics about registered and verified art pieces
Materials Tracking: Documents recycled materials used in each art installation
Development Plan
Smart Contract Development
Implement core data structures (ArtPiece, PlatformStats)
Develop registration function for artists to submit new pieces
Create verification system for curators
Add query functions for retrieving art information and platform statistics
Testing & Security Audit
Unit testing for all contract functions
Integration testing with mock frontend
Security audit to identify potential vulnerabilities
Frontend Development
Create user interface for artists to register art pieces
Develop curator dashboard for verification process
Build public gallery to browse and learn about art installations
Integration & Testing
Connect frontend to smart contract functions
End-to-end testing of full application flow
User acceptance testing with artists and curators
Deployment
Deploy smart contract to mainnet
Launch web application
Begin onboarding artists and curators
Personal Story
As an artist and environmental activist, I've always been drawn to the transformative potential of discarded materials. Growing up in a community where waste management was a challenge, I witnessed firsthand how creative repurposing could give new life to what others considered trash. This project was born from my desire to formalize and incentivize this practice, creating a platform where artists like myself could showcase their upcycled creations while making a measurable environmental impact. Upcycled Art Installation represents my commitment to combining technology, art, and sustainability into a powerful force for positive change.

# Installation Guide
Prerequisites
Node.js v16 or later
Rust and Cargo
Soroban CLI
Git
# Setup Instructions
Clone the repository
git clone https://github.com/lsap1111/Upcycled_Art_Installation.git
cd Upcycled_Art_Installation
Install dependencies
npm install
Build the smart contract
cargo build --target wasm32-unknown-unknown --release
Deploy to local development network
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/upcycled_art.wasm
Start the development server
npm run dev
Open your browser and navigate to http://localhost:3000
Environment Variables
Create a .env file in the root directory with the following variables:

NETWORK_PASSPHRASE=Test SDF Network ; September 2015
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
SOROBAN_NETWORK_PASSPHRASE=Test SDF Network ; September 2015
Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

# Fork the project
Create your feature branch (git checkout -b feature/amazing-feature)
Commit your changes (git commit -m 'Add some amazing feature')
Push to the branch (git push origin feature/amazing-feature)
Open a Pull Request

#License
This project is licensed under the MIT License - see the LICENSE file for details.

Contact
Aayam Pawar - @aayampawar - aayam.pawar@example.com

# Project Link: https://github.com/lsap1111/Upcycled_Art_Installation
![project img](asd.png)
