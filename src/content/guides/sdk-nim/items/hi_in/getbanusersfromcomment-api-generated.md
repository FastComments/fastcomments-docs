## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[GetBannedUsersFromCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_from_comment_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getBanUsersFromComment उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getBanUsersFromComment(tenantId = "my-tenant-001", commentId = "cmt-123456", sso = "")
if response.isSome:
  let banInfo = response.get()
  echo banInfo
[inline-code-end]