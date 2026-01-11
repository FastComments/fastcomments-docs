### Uruchamianie testów jednostkowych

Testy jednostkowe obejmują funkcjonalność SSO:

```bash
swift test --filter SSOTests
```

### Uruchamianie testów integracyjnych

Testy integracyjne wymagają ustawienia zmiennych środowiskowych:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```