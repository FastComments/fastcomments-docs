The FastComments Swift SDK consists of several modules:

- **Client Module** - Auto-generated API client for FastComments REST APIs
  - Complete type definitions for all API models
  - Both authenticated (`DefaultAPI`) and public (`PublicAPI`) endpoints
  - Full async/await support
  - See [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) for detailed API documentation

- **SSO Module** - Server-side Single Sign-On utilities
  - Secure token generation for user authentication
  - Support for both simple and secure SSO modes
  - HMAC-SHA256 based token signing using CryptoKit