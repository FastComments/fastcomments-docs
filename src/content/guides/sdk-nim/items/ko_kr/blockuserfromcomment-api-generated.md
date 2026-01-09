## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| blockFromCommentParams | BlockFromCommentParams | 아니요 |  |
| userId | string | 아니요 |  |
| anonUserId | string | 아니요 |  |

## 응답

반환: [`Option[BlockFromCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_from_comment_public200response.nim)

## 예제

[inline-code-attrs-start title = 'blockUserFromComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-98765",
  blockFromCommentParams = BlockFromCommentParams(),
  userId = "user-456",
  anonUserId = ""
)
if response.isSome:
  let blocked = response.get()
  echo "Block confirmed for tenant:", " my-tenant-123"
[inline-code-end]

---