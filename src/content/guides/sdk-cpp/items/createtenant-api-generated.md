## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantBody | CreateTenantBody | Yes |  |

## Response

Returns: [`CreateTenant_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenant_200_response.h)

## Example

[inline-code-attrs-start title = 'createTenant Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateTenantBody createTenantBody;
createTenantBody.name = U("My Tenant Inc.");
createTenantBody.adminEmail = boost::optional<utility::string_t>(U("admin@mytenant.com"));
auto createTask = api->createTenant(tenantId, createTenantBody)
    .then([](pplx::task<std::shared_ptr<CreateTenant_200_response>> t) -> std::shared_ptr<CreateTenant_200_response> {
        try {
            auto resp = t.get();
            if (resp) return resp;
            return std::make_shared<CreateTenant_200_response>();
        } catch (...) {
            return std::make_shared<CreateTenant_200_response>();
        }
    });
[inline-code-end]
