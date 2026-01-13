## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 아니요 |  |
| userId | string | 아니요 |  |
| anonUserId | string | 아니요 |  |

## 응답

반환: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## 예제

[inline-code-attrs-start title = 'unBlockUserFromComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "news-site-001",
  id = "cmt-8fj3k9",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-98765",
  anonUserId = ""
)

if response.isSome:
  let unblocked = response.get()
  discard unblocked
[inline-code-end]

---