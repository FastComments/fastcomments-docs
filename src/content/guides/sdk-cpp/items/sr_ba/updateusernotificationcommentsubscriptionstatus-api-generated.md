Omogući ili onemogući obavijesti za određeni komentar.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| notificationId | string | Da |  |
| optedInOrOut | string | Da |  |
| commentId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationCommentSubscriptionStatusResponse.h)

## Primjer

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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