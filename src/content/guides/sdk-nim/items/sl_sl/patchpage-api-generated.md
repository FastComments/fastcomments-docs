## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateAPIPageData | UpdateAPIPageData | No |  |

## Odgovor

Vrne: [`Option[PatchPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_page_api_response.nim)

## Primer

[inline-code-attrs-start title = 'patchPage Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchPage(
  tenantId = "my-tenant-123",
  id = "news/article-456",
  updateAPIPageData = UpdateAPIPageData(title = "Updated article title", description = "Revised description")
)

if response.isSome:
  let resp = response.get()
[inline-code-end]