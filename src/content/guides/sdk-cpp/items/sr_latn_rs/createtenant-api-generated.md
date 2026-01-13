## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createTenantBody | CreateTenantBody | Da |  |

## Odgovor

Vraća: [`CreateTenant_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenant_200_response.h)

## Primer

[inline-code-attrs-start title = 'Primer createTenant'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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