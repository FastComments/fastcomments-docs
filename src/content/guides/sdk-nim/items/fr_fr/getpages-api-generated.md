## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |

## Réponse

Retourne : [`Option[GetPagesAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pages_api_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getPages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pagesOpt, httpResp) = client.getPages(tenantId = "my-tenant-123")
if pagesOpt.isSome:
  let pages = pagesOpt.get()
  echo pages
else:
  echo "No pages returned"
echo httpResp
[inline-code-end]