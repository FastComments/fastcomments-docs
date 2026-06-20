---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| page | float64 | Nein |  |
| count | float64 | Nein |  |
| textSearch | string | Nein |  |
| byIPFromComment | string | Nein |  |
| filters | string | Nein |  |
| searchFilters | string | Nein |  |
| sorts | string | Nein |  |
| demo | bool | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getApiComments Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getApiComments(
  page = 1.0,
  count = 25.0,
  textSearch = "opinion on climate summit",
  byIPFromComment = "198.51.100.23",
  filters = "status:approved",
  searchFilters = "section:world",
  sorts = "-createdAt",
  demo = false,
  sso = "sso-user-982bf"
)

if response.isSome:
  let commentsResp = response.get()
  echo "Retrieved comments response"
else:
  echo "No comments returned, HTTP status: ", httpResponse.status
[inline-code-end]

---