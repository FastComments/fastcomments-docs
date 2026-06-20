## パラメータ

| Name | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserResponse.h)

## 例

[inline-code-attrs-start title = 'getUser の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t userId = utility::conversions::to_string_t("user@example.com");
boost::optional<utility::string_t> ifNoneMatch = boost::optional<utility::string_t>(utility::conversions::to_string_t("W/\"etag-98765\""));
api->getUser(tenantId, userId)
    .then([](pplx::task<std::shared_ptr<GetUserResponse>> task){
        try {
            auto resp = task.get();
            if (resp) {
                auto clone = std::make_shared<GetUserResponse>(*resp);
            }
        } catch (...) {
        }
    });
[inline-code-end]