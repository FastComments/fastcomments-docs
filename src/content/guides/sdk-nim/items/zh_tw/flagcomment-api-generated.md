## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## 範例

[inline-code-attrs-start title = 'flagComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(
  tenantId = "my-tenant-123",
  id = "cmt-98765",
  userId = "user-12345",
  anonUserId = ""
)

if response.isSome:
  let flagResp = response.get()
  echo "Flag response received"
else:
  echo "No flag response returned"
[inline-code-end]

---