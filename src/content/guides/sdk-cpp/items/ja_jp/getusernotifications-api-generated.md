## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | const GetUserNotificationsOptions& | はい |  |

## レスポンス

戻り値: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## 例

[inline-code-attrs-start title = 'getUserNotifications の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
GetUserNotificationsOptions options;
options.limit = boost::optional<int>(20);
options.unreadOnly = boost::optional<bool>(true);
api->getUserNotifications(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetMyNotificationsResponse>> task){
        auto resp = task.get();
        auto notifications = std::make_shared<GetMyNotificationsResponse>(*resp);
    });
[inline-code-end]

---