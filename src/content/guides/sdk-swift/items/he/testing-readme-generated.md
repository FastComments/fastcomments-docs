### הרצת בדיקות יחידה

בדיקות היחידה מכסות את פונקציונליות ה-SSO:

```bash
swift test --filter SSOTests
```

### הרצת בדיקות אינטגרציה

בדיקות האינטגרציה דורשות שהמשתני סביבה יהיו מוגדרים:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```