## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| createTenantBody | CreateTenantBody | 예 |  |

## 응답

반환: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantResponse.h)

## 예제

[inline-code-attrs-start title = 'createTenant 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto bodyPtr = std::make_shared<CreateTenantBody>();
bodyPtr->setName(utility::string_t(U("Acme Corporation")));
bodyPtr->setAdminEmail(utility::string_t(U("admin@acme.com")));
bodyPtr->setSupportEmail(boost::optional<utility::string_t>(utility::string_t(U("support@acme.com"))));
bodyPtr->setPlan(boost::optional<utility::string_t>(utility::string_t(U("pro"))));
api->createTenant(tenantId, *bodyPtr).then([](pplx::task<std::shared_ptr<CreateTenantResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto newTenantId = resp->getTenantId();
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---