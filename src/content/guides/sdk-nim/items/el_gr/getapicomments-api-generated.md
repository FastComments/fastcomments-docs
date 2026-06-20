## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | float64 | Όχι |  |
| count | float64 | Όχι |  |
| textSearch | string | Όχι |  |
| byIPFromComment | string | Όχι |  |
| filters | string | Όχι |  |
| searchFilters | string | Όχι |  |
| sorts | string | Όχι |  |
| demo | bool | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getApiComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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