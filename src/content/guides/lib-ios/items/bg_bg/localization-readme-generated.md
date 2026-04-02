Предайте код на локал в конфигурацията, за да локализирате низовете, предоставени от сървъра:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    locale: "fr_fr"
)
```

Низовете на потребителския интерфейс на клиентската страна използват локализация, базирана на iOS bundle.