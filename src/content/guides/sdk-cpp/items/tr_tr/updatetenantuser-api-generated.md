---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateTenantUserBody | UpdateTenantUserBody | Evet |  |
| updateComments | string | Hayır |  |

## Yanıt

Dönen değer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Örnek

[inline-code-attrs-start title = 'updateTenantUser Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---