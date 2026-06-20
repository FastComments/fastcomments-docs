## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## 예제

[inline-code-attrs-start title = 'unBlockCommentPublic 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-987654321", publicBlockFromCommentParams = PublicBlockFromCommentParams(), sso = "")
if response.isSome:
  let unblockResult = response.get()
  discard unblockResult
else:
  discard httpResponse
[inline-code-end]

---