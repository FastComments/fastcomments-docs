## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateNotificationBody | UpdateNotificationBody | Yes |  |
| userId | string | No |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'updateNotification の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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