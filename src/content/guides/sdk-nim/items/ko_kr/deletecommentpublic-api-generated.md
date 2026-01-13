## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 아니요 |  |
| editKey | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[DeleteCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_public200response.nim)

## 예제

[inline-code-attrs-start title = 'deleteCommentPublic 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  broadcastId = "",
  editKey = "",
  sso = ""
)
if response.isSome:
  let deleted = response.get()
  echo "Delete succeeded"
  echo "HTTP status: ", httpResponse.status
else:
  echo "Delete failed, HTTP status: ", httpResponse.status
[inline-code-end]

---