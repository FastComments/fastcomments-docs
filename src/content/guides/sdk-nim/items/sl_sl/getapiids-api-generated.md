## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| textSearch | string | Ne |  |
| byIPFromComment | string | Ne |  |
| filters | string | Ne |  |
| searchFilters | string | Ne |  |
| afterId | string | Ne |  |
| demo | bool | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getApiIds'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getApiIds(
  textSearch = "urgent moderation review",
  byIPFromComment = "203.0.113.45",
  filters = "status:pending,flagged",
  searchFilters = "author:jane.doe@example.com",
  afterId = "cmt_9f8e7d6a",
  demo = false,
  sso = "sso-token-6b7f9a"
)

if response.isSome:
  let idsResp = response.get()
  echo idsResp
[inline-code-end]

---