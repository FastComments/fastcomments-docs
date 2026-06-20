## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| broadcastId | string | Όχι |  |
| editKey | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[PublicAPIDeleteCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_delete_comment_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-987654321", broadcastId = "", editKey = "", sso = "")
if response.isSome:
  let deleted = response.get()
  echo "Delete acknowledged, HTTP status: ", httpResponse.status
[inline-code-end]