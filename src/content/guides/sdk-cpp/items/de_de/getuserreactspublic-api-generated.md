## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postIds | vector<string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UserReactsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getUserReactsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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