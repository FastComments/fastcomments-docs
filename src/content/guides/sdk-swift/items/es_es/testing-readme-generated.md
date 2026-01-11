### Ejecutar pruebas unitarias

Las pruebas unitarias cubren la funcionalidad SSO:

```bash
swift test --filter SSOTests
```

### Ejecutar pruebas de integración

Las pruebas de integración requieren que se establezcan variables de entorno:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```