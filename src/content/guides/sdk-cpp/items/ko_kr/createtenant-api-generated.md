## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantBody | CreateTenantBody | Yes |  |

## 응답

반환: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantResponse.h)

## 예시

[inline-code-attrs-start title = 'createTenant 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
CreateTenantBody body;
body.setName(utility::conversions::to_string_t("Acme Corp"));
body.setAdminEmail(utility::conversions::to_string_t("admin@acme.com"));
body.setPlan(utility::conversions::to_string_t("enterprise"));
body.setDescription(boost::optional<utility::string_t>(utility::conversions::to_string_t("Primary tenant for Acme")));

api->createTenant(tenantId, body).then([](pplx::task<std::shared_ptr<CreateTenantResponse>> t){
    auto resp = t.get();
});
[inline-code-end]