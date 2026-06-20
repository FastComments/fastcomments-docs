---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vraća: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteV2PageReact'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteV2PageReact(tenantId = "my-tenant-123", urlId = "news/2026/politics-election", id = "react-456")
if response.isSome:
  let react = response.get()
  echo react
else:
  echo "No reaction returned, status: ", httpResponse.status
[inline-code-end]

---