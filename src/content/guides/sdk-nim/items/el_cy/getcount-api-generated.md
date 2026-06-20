## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| textSearch | string | Όχι |  |
| byIPFromComment | string | Όχι |  |
| filter | string | Όχι |  |
| searchFilters | string | Όχι |  |
| demo | bool | Όχι |  |
| sso | string | Όχι |  |

## Response

Επιστρέφει: [`Option[ModerationAPICountCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_count_comments_response.nim)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCount(
  textSearch = "climate change",
  byIPFromComment = "203.0.113.5",
  filter = "status:approved",
  searchFilters = "author:john.doe@example.com;tag:opinion",
  demo = false,
  sso = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
)
if response.isSome:
  let countResp = response.get()
  discard countResp
  echo "Count response received"
else:
  echo "No count data returned"
[inline-code-end]