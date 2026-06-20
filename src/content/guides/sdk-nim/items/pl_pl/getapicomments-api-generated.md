## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| page | float64 | Nie |  |
| count | float64 | Nie |  |
| textSearch | string | Nie |  |
| byIPFromComment | string | Nie |  |
| filters | string | Nie |  |
| searchFilters | string | Nie |  |
| sorts | string | Nie |  |
| demo | bool | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getApiComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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