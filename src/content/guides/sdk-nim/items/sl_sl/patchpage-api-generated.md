## Parametri

| Name | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updateAPIPageData | UpdateAPIPageData | Ne |  |

## Odgovor

Vrača: [`Option[PatchPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_page_api_response.nim)

## Primer

[inline-code-attrs-start title = 'patchPage Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateData = UpdateAPIPageData(
  title = "Breaking: Major Event Update",
  urlId = "news/major-event-update",
  visible = true,
  tags = @["breaking", "headline"],
  sortOrder = 5
)

let (response, httpResponse) = client.patchPage(
  tenantId = "my-tenant-123",
  id = "news/major-event-update",
  updateAPIPageData = updateData
)

if response.isSome:
  let page = response.get()
  discard page
[inline-code-end]

---