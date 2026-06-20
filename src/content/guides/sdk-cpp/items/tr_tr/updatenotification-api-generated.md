## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateNotificationBody | UpdateNotificationBody | Evet |  |
| userId | string | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Örnek

[inline-code-attrs-start title = 'updateNotification Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t notificationId = utility::conversions::to_string_t("notif-456");
auto updateBodyPtr = std::make_shared<UpdateNotificationBody>();
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(utility::conversions::to_string_t("user@example.com"));
api->updateNotification(tenantId, notificationId, *updateBodyPtr, userId)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
        try {
            auto resp = task.get();
            (void)resp;
        } catch (...) {
        }
    });
[inline-code-end]

---