## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| textSearch | string | Nej |  |
| byIPFromComment | string | Nej |  |
| filters | string | Nej |  |
| searchFilters | string | Nej |  |
| afterId | string | Nej |  |
| demo | bool | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getApiIds Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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