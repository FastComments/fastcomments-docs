## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | Yes |  |

## 응답

반환값: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APICreateUserBadgeResponse.h)

## 예제

[inline-code-attrs-start title = 'createUserBadge 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---