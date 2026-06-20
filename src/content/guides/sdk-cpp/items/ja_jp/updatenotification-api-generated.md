---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateNotificationBody | UpdateNotificationBody | はい |  |
| userId | string | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'updateNotification の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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