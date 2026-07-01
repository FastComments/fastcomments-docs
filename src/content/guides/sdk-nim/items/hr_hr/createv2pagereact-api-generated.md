## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | No |  |
| title | string = "" | No |  |

## Odgovor

Vraća: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer createV2PageReact'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pageResult, httpResponse) = client.createV2PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  id = "page-456",
  title = "Breaking News",
)

if pageResult.isSome:
  let page = pageResult.get()
  # koristite `page` prema potrebi
[inline-code-end]