## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| email | string | Yes |  |

## Response

Returns: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByEmailAPIResponse.h)

## Example

[inline-code-attrs-start title = 'getSSOUserByEmail Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t email = U("user@example.com");
boost::optional<utility::string_t> emailOpt = email;
api->getSSOUserByEmail(tenantId, emailOpt.value()).then([](std::shared_ptr<GetSSOUserByEmailAPIResponse> resp) {
    auto safeResp = resp ? resp : std::make_shared<GetSSOUserByEmailAPIResponse>();
    std::cout << "SSO user fetch " << (resp ? "succeeded" : "failed") << std::endl;
}).wait();
[inline-code-end]
