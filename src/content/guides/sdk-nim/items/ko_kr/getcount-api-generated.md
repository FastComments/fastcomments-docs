## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| textSearch | string | 아니오 |  |
| byIPFromComment | string | 아니오 |  |
| filter | string | 아니오 |  |
| searchFilters | string | 아니오 |  |
| demo | bool | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[ModerationAPICountCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_count_comments_response.nim)

## 예제

[inline-code-attrs-start title = 'getCount 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---