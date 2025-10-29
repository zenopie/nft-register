# NFT Register - Secret Network SNIP721 Minter Contract

## Overview

A Secret Network smart contract for registering scientists and minting SNIP721 NFTs with both public and private metadata. This contract serves as a business logic layer that interacts with a SNIP721 NFT contract to mint verification tokens for scientists on the Unbound Science platform.

## Features

- **Scientist Registration**: Register scientists with their professional details
- **NFT Minting**: Automatically mints SNIP721 NFTs upon registration
- **Public Metadata**: Publicly visible scientist information (name, institution, specialization)
- **Private Metadata**: Confidential information (email, ORCID, wallet address, research description)
- **Unique Token IDs**: Timestamp-based unique identifiers for each scientist token

## Contract Functions

### Instantiate
Initialize the contract with:
- Owner address
- SNIP721 contract address
- SNIP721 contract code hash

### Execute Messages

#### RegisterScientist
Register a scientist and mint their NFT:
- `name`: Scientist's full name
- `institution`: Academic or research institution
- `specialization`: Field of expertise
- `description`: Research focus and background
- `email`: Contact email
- `image`: Optional profile image URL
- `website`: Optional personal/research website
- `orcid_id`: Optional ORCID identifier

#### UpdateConfig
Update contract configuration (owner only)

### Query Messages

#### GetConfig
Returns current contract configuration

## Building

### Development Build
```bash
make build
```

### Mainnet Reproducible Build
```bash
make build-mainnet-reproducible
```

This uses the Secret Network contract optimizer Docker image to produce a reproducible build suitable for mainnet deployment.

## NFT Metadata Structure

### Public Metadata
- Name: "{Name} - Scientist"
- Description: "Scientist verification token on Secret Network"
- Image: Optional profile image URL
- Attributes:
  - Type: "Scientist"
  - Institution
  - Specialization
  - Network: "Secret Network Mainnet"
  - Platform: "Unbound Science"

### Private Metadata
- Registered timestamp
- Wallet address
- Research description
- Platform: "Unbound Science"
- Email
- Website (optional)
- ORCID ID (optional)

## Dependencies

- `secret-cosmwasm-std`: Secret Network CosmWasm standard library
- `secret-toolkit`: Secret Network development toolkit
- `serde`: Serialization/deserialization
- `schemars`: JSON schema generation

## License

See LICENSE file for details.