## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| addDomainConfigParams | AddDomainConfigParams | Yes |  |

## Response

Returns: [`AddDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddDomainConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'addDomainConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
AddDomainConfigParams addDomainConfigParams;
addDomainConfigParams.domain = U("comments.example.com");
addDomainConfigParams.contactEmail = boost::optional<utility::string_t>(U("admin@example.com"));
addDomainConfigParams.allowSubdomains = boost::optional<bool>(true);
std::shared_ptr<AddDomainConfig_200_response> createdResponse;
api->addDomainConfig(tenantId, addDomainConfigParams)
.then([&createdResponse](std::shared_ptr<AddDomainConfig_200_response> resp){
    createdResponse = resp;
    return resp;
});
[inline-code-end]
