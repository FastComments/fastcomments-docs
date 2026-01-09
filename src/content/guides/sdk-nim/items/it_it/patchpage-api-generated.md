## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |
| updateAPIPageData | UpdateAPIPageData | No |  |

## Risposta

Restituisce: [`Option[PatchPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_page_api_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di patchPage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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