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
auto params = std::make_shared<AddDomainConfigParams>();
params->setDomain(U("comments.example.com"));
params->setOwnerEmail(utility::string_t(U("admin@example.com")));
params->setAllowSubdomains(boost::optional<bool>(true));
params->setAllowedOrigins(std::vector<utility::string_t>{U("https://www.example.com"), U("https://blog.example.com")});
api->addDomainConfig(tenantId, *params)
    .then([](pplx::task<std::shared_ptr<AddDomainConfig_200_response>> task) {
        try {
            auto resp = task.get();
            (void)resp;
        } catch (const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]
