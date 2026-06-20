## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateTenantUserBody | UpdateTenantUserBody | Da |  |
| updateComments | string | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer updateTenantUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto userId = utility::string_t(U("user@example.com"));
auto bodyPtr = std::make_shared<UpdateTenantUserBody>();
boost::optional<utility::string_t> updateComments = boost::optional<utility::string_t>(U("true"));
api->updateTenantUser(tenantId, userId, *bodyPtr, updateComments)
.then([](std::shared_ptr<APIEmptyResponse> resp) {
    if (resp) std::cout << "Tenant user updated successfully\n";
    else std::cout << "No response from updateTenantUser\n";
});
[inline-code-end]