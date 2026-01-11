### Unit-Tests ausführen

Unit-Tests decken die SSO-Funktionalität ab:

```bash
swift test --filter SSOTests
```

### Integrationstests ausführen

Für Integrationstests müssen Umgebungsvariablen gesetzt werden:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```