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
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(U("en-US"));
api->getSSOUserById(tenantId, userId).then([](std::shared_ptr<GetSSOUserByIdAPIResponse> resp) {
    if (!resp) {
        return std::make_shared<GetSSOUserByIdAPIResponse>();
    }
    return resp;
}).wait();
[inline-code-end]
