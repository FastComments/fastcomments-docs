## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| updatableCommentParams | UpdatableCommentParams | 아니오 |  |
| contextUserId | string | 아니오 |  |
| doSpamCheck | bool | 아니오 |  |
| isLive | bool | 아니오 |  |

## 응답

반환: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 예제

[inline-code-attrs-start title = 'updateComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updatableCommentParams = UpdatableCommentParams(content: "Fixed a typo in the second paragraph", tags: @["article-edit", "typo"], isApproved: true)
let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  updatableCommentParams = updatableCommentParams,
  contextUserId = "user-789",
  doSpamCheck = true,
  isLive = true
)
if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

---