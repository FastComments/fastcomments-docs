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
boost::optional<utility::string_t> optionalEmail = U("jane.doe@example.com");
utility::string_t email = optionalEmail ? *optionalEmail : U("jane.doe@fallback.com");
api->getSSOUserByEmail(tenantId, email)
    .then([](pplx::task<std::shared_ptr<GetSSOUserByEmailAPIResponse>> task){
        try {
            auto resp = task.get();
            auto userCopy = std::make_shared<GetSSOUserByEmailAPIResponse>(*resp);
            (void)userCopy;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
