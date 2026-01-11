### Esecuzione dei test unitari

I test unitari coprono la funzionalità SSO:

```bash
swift test --filter SSOTests
```

### Esecuzione dei test di integrazione

I test di integrazione richiedono che le variabili d'ambiente siano impostate:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```