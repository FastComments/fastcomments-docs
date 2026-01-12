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
boost::optional<utility::string_t> tenantOpt(U("my-tenant-123"));
utility::string_t tenantId = tenantOpt.value_or(U("my-tenant-123"));
utility::string_t email = U("alice@example.com");

api->getSSOUserByEmail(tenantId, email)
    .then([](pplx::task<std::shared_ptr<GetSSOUserByEmailAPIResponse>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                auto userCopy = std::make_shared<GetSSOUserByEmailAPIResponse>(*resp);
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
