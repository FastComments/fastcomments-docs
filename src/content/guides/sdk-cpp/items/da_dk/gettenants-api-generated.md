## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetTenantsOptions& | Yes |  |

## Svar

Returnerer: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantsResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getTenants Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetTenantsOptions options;
options.includeDeleted = boost::make_optional(false);
options.searchTerm = boost::make_optional(utility::string_t(U("enterprise")));

api->getTenants(utility::string_t(U("my-tenant-123")), options)
    .then([](std::shared_ptr<GetTenantsResponse> response) {
    })
    .then([](pplx::task<void> t){ try{ t.get(); }catch(...){} });
[inline-code-end]