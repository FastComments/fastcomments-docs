## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | No |  |
| title | string = "" | No |  |

## Απάντηση

Επιστρέφει: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'createV2PageReact Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pageResult, httpResponse) = client.createV2PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  id = "page-456",
  title = "Breaking News",
)

if pageResult.isSome:
  let page = pageResult.get()
  # χρησιμοποιήστε `page` όπως χρειάζεται
[inline-code-end]