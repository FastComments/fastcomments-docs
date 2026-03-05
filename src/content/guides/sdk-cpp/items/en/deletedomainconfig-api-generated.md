## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Response

Returns: [`DeleteDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteDomainConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteDomainConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t(U("my-tenant-123"));
utility::string_t domain = utility::string_t(U("comments.example.com"));
boost::optional<utility::string_t> requestId = utility::string_t(U("trace-789"));
auto fallback = std::make_shared<DeleteDomainConfig_200_response>();
api->deleteDomainConfig(tenantId, domain)
    .then([fallback, requestId](pplx::task<std::shared_ptr<DeleteDomainConfig_200_response>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                *fallback = *resp;
            }
        } catch (const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]
