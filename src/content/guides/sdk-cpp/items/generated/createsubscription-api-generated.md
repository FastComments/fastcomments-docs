## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Yes |  |

## Response

Returns: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateSubscriptionAPIResponse.h)

## Example

[inline-code-attrs-start title = 'createSubscription Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateAPIUserSubscriptionData subscriptionData;
subscriptionData.userEmail = U("user@example.com");
subscriptionData.planId = U("pro-monthly");
subscriptionData.autoRenew = boost::optional<bool>(true);
subscriptionData.couponCode = boost::optional<utility::string_t>(U("WELCOME10"));
auto task = api->createSubscription(tenantId, subscriptionData)
.then([=](pplx::task<std::shared_ptr<CreateSubscriptionAPIResponse>> t) {
    try {
        return t.get();
    } catch (const std::exception&) {
        return std::make_shared<CreateSubscriptionAPIResponse>();
    }
});
[inline-code-end]
