## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| page | float64 | Nee |  |
| count | float64 | Nee |  |
| textSearch | string | Nee |  |
| byIPFromComment | string | Nee |  |
| filters | string | Nee |  |
| searchFilters | string | Nee |  |
| sorts | string | Nee |  |
| demo | bool | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getApiComments Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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