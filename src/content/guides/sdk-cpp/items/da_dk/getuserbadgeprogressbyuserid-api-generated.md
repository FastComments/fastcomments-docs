## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Ja |  |

## Svar

Returnerer: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeProgressResponse.h)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getUserBadgeProgressByUserId'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> locale;
api->getUserBadgeProgressByUserId(tenantId, userId)
.then([=](pplx::task<std::shared_ptr<APIGetUserBadgeProgressResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<APIGetUserBadgeProgressResponse>();
        return resp;
    } catch (...) {
        return std::shared_ptr<APIGetUserBadgeProgressResponse>(nullptr);
    }
})
.then([](std::shared_ptr<APIGetUserBadgeProgressResponse> resp) {
    (void)resp;
});
[inline-code-end]

---