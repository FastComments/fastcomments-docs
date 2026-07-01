## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateNotificationBody | UpdateNotificationBody | 예 |  |
| userId | string | 아니오 |  |

## 응답

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'updateNotification 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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