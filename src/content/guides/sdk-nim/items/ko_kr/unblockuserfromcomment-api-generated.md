## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 아니오 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |

## 응답

반환: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## 예제

[inline-code-attrs-start title = 'unBlockUserFromComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-9f3b2a",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-1024",
  anonUserId = "anon-77b"
)

if response.isSome:
  let unblockResult = response.get()
  echo unblockResult
else:
  echo "Unblock failed"
[inline-code-end]

---