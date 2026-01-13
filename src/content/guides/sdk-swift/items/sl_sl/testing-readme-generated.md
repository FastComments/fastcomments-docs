### Zaganjanje enotskih testov

Enotski testi pokrivajo funkcionalnost SSO:

```bash
swift test --filter SSOTests
```

### Zaganjanje integracijskih testov

Integracijski testi zahtevajo, da so nastavljene spremenljivke okolja:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```