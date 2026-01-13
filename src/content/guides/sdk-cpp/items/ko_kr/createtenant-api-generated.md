## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createTenantBody | CreateTenantBody | 예 |  |

## 응답

반환: [`CreateTenant_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenant_200_response.h)

## 예제

[inline-code-attrs-start title = 'createTenant 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateTenantBody createTenantBody;
createTenantBody.displayName = U("My Tenant Inc");
createTenantBody.adminEmail = U("admin@mytenant.com");
createTenantBody.plan = boost::optional<utility::string_t>(U("pro"));
auto task = api->createTenant(tenantId, createTenantBody)
    .then([](std::shared_ptr<CreateTenant_200_response> resp){
        if (resp){
            auto createdId = std::make_shared<utility::string_t>(resp->tenantId);
        }
    });
task.wait();
[inline-code-end]

---