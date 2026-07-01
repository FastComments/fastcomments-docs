Enable or disable notifications for a specific comment.

## Parameters

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|-------------|------------|
| tenantId | string | Ναι |  |
| notificationId | string | Ναι |  |
| optedInOrOut | string | Ναι |  |
| commentId | string | Ναι |  |
| sso | string | Όχι |  |

## Response

Επιστρέφει: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationCommentSubscriptionStatusResponse.h)

## Example

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Παράδειγμα'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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