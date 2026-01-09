הגדר את משתני הסביבה הנדרשים:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

הרץ את הבדיקות:

```bash
nimble test
```

או הרץ בדיקות ספציפיות:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```