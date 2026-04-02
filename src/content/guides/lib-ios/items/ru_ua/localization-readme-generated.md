Передайте код локали в конфигурации, чтобы локализовать строки, предоставляемые сервером:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    locale: "fr_fr"
)
```

Строки интерфейса на стороне клиента используют локализацию, основанную на iOS-бандлах.