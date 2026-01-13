---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUser_200_response.h)

## 例

[inline-code-attrs-start title = 'getUser の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenant(U("my-tenant-123"));
boost::optional<utility::string_t> userId(U("user@example.com"));
api->getUser(tenant.value(), userId.value())
    .then([](pplx::task<std::shared_ptr<GetUser_200_response>> task) {
        try {
            auto resp = task.get();
            if (resp) {
                auto result_copy = std::make_shared<GetUser_200_response>(*resp);
            } else {
                auto fallback = std::make_shared<GetUser_200_response>();
            }
        } catch (const std::exception &e) {
        }
    });
[inline-code-end]

---