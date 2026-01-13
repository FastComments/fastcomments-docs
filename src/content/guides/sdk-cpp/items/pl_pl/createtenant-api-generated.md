---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createTenantBody | CreateTenantBody | Tak |  |

## Odpowiedź

Zwraca: [`CreateTenant_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenant_200_response.h)

## Przykład

[inline-code-attrs-start title = 'Przykład createTenant'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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