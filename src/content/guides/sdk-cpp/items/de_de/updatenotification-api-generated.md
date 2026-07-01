## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateNotificationBody | UpdateNotificationBody | Ja |  |
| userId | string | Nein |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Beispiel

[inline-code-attrs-start title = 'updateNotification Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto updateBody = std::make_shared<UpdateNotificationBody>();
updateBody->title = utility::conversions::to_string_t("System Maintenance");
updateBody->message = utility::conversions::to_string_t("Scheduled downtime at 02:00 UTC.");
api->updateNotification(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("notif-456"),
    updateBody,
    boost::optional<utility::string_t>(utility::conversions::to_string_t("admin-user"))
).then([](std::shared_ptr<APIEmptyResponse>){});
[inline-code-end]