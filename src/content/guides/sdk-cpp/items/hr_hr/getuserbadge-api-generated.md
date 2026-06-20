## Parametri

| Naziv | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeResponse.h)

## Primjer

[inline-code-attrs-start title = 'getUserBadge Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userOpt(U("alice@example.com"));
utility::string_t userId = userOpt.value_or(U("alice@example.com"));
auto ctx = std::make_shared<utility::string_t>(U("request-context-1"));
api->getUserBadge(tenantId, userId)
.then([ctx](pplx::task<std::shared_ptr<APIGetUserBadgeResponse>> task) {
    try {
        auto resp = task.get();
        if (resp) {
            *ctx = U("badge-retrieved");
        }
    } catch (const std::exception &e) {
        *ctx = U("error");
    }
});
[inline-code-end]

---