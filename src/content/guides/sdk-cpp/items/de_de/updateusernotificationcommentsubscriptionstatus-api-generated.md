Aktivieren oder deaktivieren Sie Benachrichtigungen für einen bestimmten Kommentar.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| notificationId | string | Yes |  |
| optedInOrOut | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Response

Rückgabe: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationCommentSubscriptionStatusResponse.h)

## Beispiel

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto updateTask = api->updateUserNotificationCommentSubscriptionStatus(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("notif-456"),
    utility::conversions::to_string_t("optedIn"),
    utility::conversions::to_string_t("comment-789"),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("sso-token-abc"))
).then([](std::shared_ptr<UpdateUserNotificationCommentSubscriptionStatusResponse> resp){
    (void)resp;
});
[inline-code-end]

---