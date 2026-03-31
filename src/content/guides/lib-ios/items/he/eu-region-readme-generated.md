---
כדי להשתמש במרכז הנתונים של האיחוד האירופי, הגדר את השדה `region` בקונפיגורציה שלך:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

זה מנתב את כל בקשות ה-API והחיבורים של WebSocket אל `eu.fastcomments.com`.

---
---