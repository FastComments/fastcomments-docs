## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| textSearch | string | Non |  |
| byIPFromComment | string | Non |  |
| filters | string | Non |  |
| searchFilters | string | Non |  |
| afterId | string | Non |  |
| demo | bool | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getApiIds'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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