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
boost::optional<utility::string_t> tenantOpt = boost::optional<utility::string_t>(U("my-tenant-123"));
utility::string_t domain = U("blog.example.com");
api->getDomainConfig(*tenantOpt, domain)
    .then([=](std::shared_ptr<GetDomainConfig_200_response> resp){
        auto cfg = resp ? resp : std::make_shared<GetDomainConfig_200_response>();
        (void)cfg;
    });
[inline-code-end]
