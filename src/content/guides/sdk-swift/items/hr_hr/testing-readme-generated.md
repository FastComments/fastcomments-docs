### Pokretanje jedinčnih testova

Jedinčni testovi pokrivaju SSO funkcionalnost:

```bash
swift test --filter SSOTests
```

### Pokretanje integracijskih testova

Integracijski testovi zahtijevaju postavljanje varijabli okruženja:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```