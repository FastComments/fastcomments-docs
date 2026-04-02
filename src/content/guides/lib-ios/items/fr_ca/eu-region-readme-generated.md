---
Pour utiliser le centre de données de l'UE, définissez le champ `region` dans votre configuration :

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Cela achemine toutes les requêtes API et les connexions WebSocket vers `eu.fastcomments.com`.

---
---