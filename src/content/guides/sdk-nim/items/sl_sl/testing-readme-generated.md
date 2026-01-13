Nastavite zahtevane okoljske spremenljivke:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Zaženite teste:

```bash
nimble test
```

Ali pa zaženite določene teste:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```