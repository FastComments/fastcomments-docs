## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| title | string | No |  |

## Risposta

Restituisce: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di createV1PageReact'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createV1PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/2026/market-rally",
  title = "Breaking News: Markets Rally Today"
)

if response.isSome:
  let pageReact = response.get()
  echo "Page react created: ", pageReact
else:
  echo "Failed to create page react: ", httpResponse
[inline-code-end]

---