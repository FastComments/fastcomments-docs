### Pokretanje jediničnih testova

Jedinični testovi pokrivaju SSO funkcionalnost:

```bash
swift test --filter SSOTests
```

### Pokretanje integracionih testova

Integracioni testovi zahtevaju da budu postavljene promenljive okruženja:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```