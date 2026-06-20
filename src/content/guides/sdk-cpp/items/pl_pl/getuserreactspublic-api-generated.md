---
## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| postIds | vector<string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UserReactsResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserReactsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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