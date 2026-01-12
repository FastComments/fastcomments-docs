## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## Response

Returns: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSubscriptionsAPIResponse.h)

## Example

[inline-code-attrs-start title = 'getSubscriptions Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
api->getSubscriptions(tenantId, userId)
.then([](pplx::task<std::shared_ptr<GetSubscriptionsAPIResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<GetSubscriptionsAPIResponse>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]
