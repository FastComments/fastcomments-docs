## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetUserNotificationsOptions | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getUserNotifications उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getUserNotifications(tenantId = "my-tenant-123", options = GetUserNotificationsOptions())
if maybeResponse.isSome:
  let notifications = maybeResponse.get()
[inline-code-end]