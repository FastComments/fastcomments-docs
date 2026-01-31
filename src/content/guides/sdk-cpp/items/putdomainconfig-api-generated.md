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
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("comments.example.com");
UpdateDomainConfigParams updateDomainConfigParams;
updateDomainConfigParams.contactEmail = boost::optional<utility::string_t>(U("ops@example.com"));
updateDomainConfigParams.moderationEnabled = boost::optional<bool>(true);
updateDomainConfigParams.allowedOrigins = std::vector<utility::string_t>{ U("https://example.com"), U("https://admin.example.com") };
api->putDomainConfig(tenantId, domainToUpdate, updateDomainConfigParams)
.then([](std::shared_ptr<GetDomainConfig_200_response> resp){
    if(!resp) return;
    auto copy = std::make_shared<GetDomainConfig_200_response>(*resp);
    std::cout << "Updated domain config received" << std::endl;
});
[inline-code-end]
