### Executando Testes Unitários

Os testes unitários cobrem a funcionalidade SSO:

```bash
swift test --filter SSOTests
```

### Executando Testes de Integração

Os testes de integração requerem que variáveis de ambiente sejam definidas:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```