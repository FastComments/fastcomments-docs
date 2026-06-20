## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |

## Risposta

Restituisce: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteV1PageReact'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteV1PageReact(tenantId = "my-tenant-123", urlId = "news/article-title")
if response.isSome:
  let deletedReact = response.get()
  echo "Deleted react:", deletedReact
else:
  echo "No react returned for tenant: my-tenant-123, url: news/article-title"
[inline-code-end]

---