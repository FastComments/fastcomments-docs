## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUserResponse.h)

## 예제

[inline-code-attrs-start title = 'getTenantUser 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> auditReason = U("admin-request");
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("john.doe@example.com");
auto fallback = std::make_shared<GetTenantUserResponse>();
api->getTenantUser(tenantId, id).then([fallback, auditReason](pplx::task<std::shared_ptr<GetTenantUserResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = fallback;
        (void)auditReason;
    } catch(const std::exception&) {
    }
});
[inline-code-end]

---