## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createUserBadgeParams | CreateUserBadgeParams | Ja |  |

## Respons

Retourneert: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APICreateUserBadgeResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'createUserBadge Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateUserBadgeParams params;
params.userEmail = utility::string_t(U("jane.doe@example.com"));
params.badgeSlug = utility::string_t(U("community-moderator"));
params.issuedBy = utility::string_t(U("admin@my-tenant.com"));
params.expiresAt = boost::optional<utility::string_t>(utility::string_t(U("2026-12-31T23:59:59Z")));
auto task = api->createUserBadge(tenantId, params)
    .then([](pplx::task<std::shared_ptr<APICreateUserBadgeResponse>> t){
        auto resp = t.get();
        if (resp) return resp;
        return std::make_shared<APICreateUserBadgeResponse>();
    });
[inline-code-end]