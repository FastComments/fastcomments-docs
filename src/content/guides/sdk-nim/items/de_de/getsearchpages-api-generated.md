## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| options | GetSearchPagesOptions | Nein |  |

## Antwort

Rückgabe: [`Option[ModerationPageSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_page_search_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getSearchPages Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchResp, httpResp) = client.getSearchPages(
  tenantId = "my-tenant-123",
  options = GetSearchPagesOptions(
    page: 1,
    pageSize: 20,
    query: "spam"
  )
)

if searchResp.isSome:
  let resp = searchResp.get()
[inline-code-end]