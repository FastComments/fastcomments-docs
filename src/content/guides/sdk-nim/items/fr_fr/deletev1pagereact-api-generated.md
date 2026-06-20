## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |

## Réponse

Renvoie: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteV1PageReact'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteV1PageReact(tenantId = "my-tenant-123", urlId = "news/article-title")
if response.isSome:
  let deletedReact = response.get()
  echo "Deleted react:", deletedReact
else:
  echo "No react returned for tenant: my-tenant-123, url: news/article-title"
[inline-code-end]

---