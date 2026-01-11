## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'putDomainConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto domainToUpdate = utility::string_t(U("mydomain.com"));
auto updateParams = std::make_shared<UpdateDomainConfigParams>();
updateParams->enabled = true;
updateParams->contactEmail = boost::optional<utility::string_t>(U("admin@mydomain.com"));
updateParams->allowedOrigins = std::vector<utility::string_t>{ U("https://www.mydomain.com"), U("https://dashboard.mydomain.com") };

api->putDomainConfig(tenantId, domainToUpdate, *updateParams)
.then([](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            std::cout << "Domain config updated for tenant\n";
        }
    } catch (const std::exception& e) {
        std::cerr << "Failed to update domain config: " << e.what() << '\n';
    }
});
[inline-code-end]
