## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| page | float64 | 아니오 |  |
| count | float64 | 아니오 |  |
| textSearch | string | 아니오 |  |
| byIPFromComment | string | 아니오 |  |
| filters | string | 아니오 |  |
| searchFilters | string | 아니오 |  |
| sorts | string | 아니오 |  |
| demo | bool | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## 예제

[inline-code-attrs-start title = 'getApiComments 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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