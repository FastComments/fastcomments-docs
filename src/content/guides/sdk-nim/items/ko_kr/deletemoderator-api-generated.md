## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| sendEmail | string | 아니요 |  |

## 응답

반환: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 예제

[inline-code-attrs-start title = 'deleteModerator 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteModerator(tenantId = "my-tenant-123", id = "moderator-456", sendEmail = "false")
if response.isSome:
  let flagResp = response.get()
  echo "Moderator deletion response: ", $flagResp
else:
  echo "No response body; HTTP status: ", $httpResponse.status
[inline-code-end]

---