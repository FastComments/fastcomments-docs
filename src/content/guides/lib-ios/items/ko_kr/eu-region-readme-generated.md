---
EU 데이터 센터를 사용하려면 구성의 `region` 필드를 설정하세요:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

이렇게 하면 모든 API 요청과 WebSocket 연결이 `eu.fastcomments.com`으로 라우팅됩니다.

---
---