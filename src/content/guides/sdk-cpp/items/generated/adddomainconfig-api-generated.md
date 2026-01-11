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
AddDomainConfigParams params;
params.domain = utility::string_t(U("comments.example.com"));
params.contactEmail = utility::string_t(U("admin@example.com"));
params.enforceHttps = boost::optional<bool>(true);
params.notes = boost::optional<utility::string_t>(utility::string_t(U("Primary comments domain")));

api->addDomainConfig(utility::string_t(U("my-tenant-123")), params)
.then([](std::shared_ptr<AddDomainConfig_200_response> resp){
    if (!resp) return;
    auto result = std::make_shared<AddDomainConfig_200_response>(*resp);
    std::wcout << U("Domain config added for tenant: my-tenant-123") << std::endl;
});
[inline-code-end]
