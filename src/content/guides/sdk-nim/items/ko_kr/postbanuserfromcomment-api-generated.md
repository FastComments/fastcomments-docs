## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| banEmail | bool | 아니오 |  |
| banEmailDomain | bool | 아니오 |  |
| banIP | bool | 아니오 |  |
| deleteAllUsersComments | bool | 아니오 |  |
| bannedUntil | string | 아니오 |  |
| isShadowBan | bool | 아니오 |  |
| updateId | string | 아니오 |  |
| banReason | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## 예제

[inline-code-attrs-start title = 'postBanUserFromComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postBanUserFromComment(
  commentId = "cmt-8f3a1b",
  banEmail = false,
  banEmailDomain = false,
  banIP = false,
  deleteAllUsersComments = false,
  bannedUntil = "",
  isShadowBan = false,
  updateId = "",
  banReason = "",
  sso = ""
)
if response.isSome:
  let banResult = response.get()
  discard banResult
else:
  echo "No ban result returned"
[inline-code-end]

---