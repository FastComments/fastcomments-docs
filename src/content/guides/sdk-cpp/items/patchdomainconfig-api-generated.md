## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| patchDomainConfigParams | PatchDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'patchDomainConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("blog.example.com");
PatchDomainConfigParams patchDomainConfigParams;
patchDomainConfigParams.enableComments = true;
patchDomainConfigParams.allowedOrigins = std::vector<utility::string_t>{ U("https://www.example.com"), U("https://blog.example.com") };
patchDomainConfigParams.adminEmail = boost::optional<utility::string_t>(U("admin@example.com"));
auto updateTask = api->patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams)
.then([](std::shared_ptr<GetDomainConfig_200_response> resp) {
    if (!resp) return std::shared_ptr<GetDomainConfig_200_response>();
    auto updated = std::make_shared<GetDomainConfig_200_response>(*resp);
    return updated;
});
updateTask.wait();
[inline-code-end]
