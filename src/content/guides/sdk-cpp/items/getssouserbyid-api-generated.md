## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByIdAPIResponse.h)

## Example

[inline-code-attrs-start title = 'getSSOUserById Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
boost::optional<utility::string_t> sessionToken = U("session-abc123");
auto dummyPtr = std::make_shared<GetSSOUserByIdAPIResponse>();
api->getSSOUserById(tenantId, id)
    .then([=](pplx::task<std::shared_ptr<GetSSOUserByIdAPIResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) {
                std::cout << "SSO user retrieved for tenant" << std::endl;
            } else {
                std::cout << "SSO user not found" << std::endl;
            }
        } catch (const std::exception &e) {
            std::cerr << "Error fetching SSO user: " << e.what() << std::endl;
        }
    });
[inline-code-end]
