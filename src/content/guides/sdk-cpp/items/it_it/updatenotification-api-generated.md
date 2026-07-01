## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateNotificationBody | UpdateNotificationBody | Sì |  |
| userId | string | No |  |

## Risposta

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateNotification'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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