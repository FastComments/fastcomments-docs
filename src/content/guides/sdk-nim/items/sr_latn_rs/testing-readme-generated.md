Podesite potrebne promenljive okruženja:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Pokrenite testove:

```bash
nimble test
```

Ili pokrenite određene testove:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```