### Exécution des tests unitaires

Les tests unitaires couvrent la fonctionnalité SSO :

```bash
swift test --filter SSOTests
```

### Exécution des tests d'intégration

Les tests d'intégration nécessitent que des variables d'environnement soient définies :

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```