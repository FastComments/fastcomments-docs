## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetUserNotificationCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotificationCount_200_response.h)

## 例

[inline-code-attrs-start title = 'getUserNotificationCount の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
std::shared_ptr<GetUserNotificationCount_200_response> result;
api->getUserNotificationCount(tenantId, sso)
.then([&result](pplx::task<std::shared_ptr<GetUserNotificationCount_200_response>> t) {
    try {
        result = t.get();
        if (!result) result = std::make_shared<GetUserNotificationCount_200_response>();
    } catch (...) {
        result = std::make_shared<GetUserNotificationCount_200_response>();
    }
}).wait();
[inline-code-end]

---