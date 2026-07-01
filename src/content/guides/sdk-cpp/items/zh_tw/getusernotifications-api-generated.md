## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | const GetUserNotificationsOptions& | 是 |  |

## 回應

Returns: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## 範例

[inline-code-attrs-start title = '取得使用者通知範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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