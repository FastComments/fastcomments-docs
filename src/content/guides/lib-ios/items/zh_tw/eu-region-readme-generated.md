---
要使用歐盟 (EU) 資料中心，請在您的設定中將 `region` 欄位設為：

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

這會將所有 API 請求和 WebSocket 連線導向 `eu.fastcomments.com`。

---
---