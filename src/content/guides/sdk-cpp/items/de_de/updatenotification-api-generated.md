## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateNotificationBody | UpdateNotificationBody | Ja |  |
| userId | string | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateNotification'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t notificationId = U("notif-456");
UpdateNotificationBody updateNotificationBody;
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("moderator@example.com"));
api->updateNotification(tenantId, notificationId, updateNotificationBody, userId)
.then([=](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto respCopy = std::make_shared<FlagCommentPublic_200_response>(*resp);
            std::cout << "Notification updated successfully\n";
        } else {
            std::cout << "No response received\n";
        }
    } catch (const std::exception &e) {
        std::cerr << "Update failed: " << e.what() << "\n";
    }
});
[inline-code-end]

---