---
## Parametri

| Name | Type | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updateAPIPageData | UpdateAPIPageData | Ne |  |

## Odgovor

Vraća: [`Option[PatchPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_page_api_response.nim)

## Primjer

[inline-code-attrs-start title = 'patchPage Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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