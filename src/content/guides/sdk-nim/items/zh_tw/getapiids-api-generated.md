## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|----------|-------------|
| textSearch | string | 否 |  |
| byIPFromComment | string | 否 |  |
| filters | string | 否 |  |
| searchFilters | string | 否 |  |
| afterId | string | 否 |  |
| demo | bool | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## 範例

[inline-code-attrs-start title = 'getApiIds 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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