## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'resetUserNotificationCount उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (resetRespOpt, httpResp) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "user-456")
if resetRespOpt.isSome:
  let resetResp = resetRespOpt.get()
  echo resetResp
else:
  echo "Reset notification count response not available"
[inline-code-end]