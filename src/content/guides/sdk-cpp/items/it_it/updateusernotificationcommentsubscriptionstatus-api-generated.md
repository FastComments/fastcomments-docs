Abilita o disabilita le notifiche per un commento specifico.

## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|-----------|-------------|
| tenantId | string | Yes |  |
| notificationId | string | Yes |  |
| optedInOrOut | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationCommentSubscriptionStatusResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateUserNotificationCommentSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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