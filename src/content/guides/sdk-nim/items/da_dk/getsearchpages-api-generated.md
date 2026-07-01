## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| options | GetSearchPagesOptions | Nej |  |

## Svar

Returnerer: [`Option[ModerationPageSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_page_search_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getSearchPages Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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