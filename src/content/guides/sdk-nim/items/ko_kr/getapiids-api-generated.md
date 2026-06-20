## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| textSearch | string | 아니오 |  |
| byIPFromComment | string | 아니오 |  |
| filters | string | 아니오 |  |
| searchFilters | string | 아니오 |  |
| afterId | string | 아니오 |  |
| demo | bool | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## 예제

[inline-code-attrs-start title = 'getApiIds 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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