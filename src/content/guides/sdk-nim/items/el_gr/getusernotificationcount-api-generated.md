## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[GetUserNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotificationCount(tenantId = "my-tenant-123", sso = "")
if response.isSome:
  let notificationData = response.get()
  echo "Received notification data: ", $notificationData
else:
  echo "No notification data returned. HTTP response: ", $httpResponse.status
[inline-code-end]

---