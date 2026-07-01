## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| options | GetSearchPagesOptions | No |  |

## Risposta

Restituisce: [`Option[ModerationPageSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_page_search_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getSearchPages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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