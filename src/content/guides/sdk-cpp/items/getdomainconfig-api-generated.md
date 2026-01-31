## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Response

Returns: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'getDomainConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("comments.example.com");
boost::optional<utility::string_t> domainOpt(domain);
auto fallbackResp = std::make_shared<GetDomainConfig_200_response>();
api->getDomainConfig(tenantId, domainOpt.value_or(domain))
    .then([fallbackResp](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = fallbackResp;
            (void)resp;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
