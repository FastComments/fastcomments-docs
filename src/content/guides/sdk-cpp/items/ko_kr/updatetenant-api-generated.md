---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantBody | UpdateTenantBody | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'updateTenant 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("admin-user-456");
auto updateTenantBody = std::make_shared<UpdateTenantBody>();
updateTenantBody->name = U("Acme Corporation");
updateTenantBody->ownerEmail = boost::optional<utility::string_t>(U("owner@acme.com"));
updateTenantBody->isActive = boost::optional<bool>(true);
api->updateTenant(tenantId, id, updateTenantBody)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
    try {
        auto resp = t.get();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---