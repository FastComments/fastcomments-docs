Définissez les variables d'environnement requises :

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Exécutez les tests :

```bash
nimble test
```

Ou exécutez des tests spécifiques :

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```