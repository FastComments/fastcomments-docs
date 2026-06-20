## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postIds | vector<string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UserReactsResponse.h)

## 範例

[inline-code-attrs-start title = 'getUserReactsPublic 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<std::vector<utility::string_t>> postIds = std::vector<utility::string_t>{ U("post-7f3a"), U("post-b2c9") };
boost::optional<utility::string_t> sso = U("user@example.com");
api->getUserReactsPublic(tenantId, postIds, sso)
    .then([](pplx::task<std::shared_ptr<UserReactsResponse>> task) {
        try {
            auto resp = task.get();
            if (!resp) resp = std::make_shared<UserReactsResponse>();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---