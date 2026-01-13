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
auto createData = std::make_shared<CreateAPIUserSubscriptionData>();
createData->userEmail = U("user@example.com");
createData->planId = U("pro-monthly");
createData->paymentMethod = U("card_visa_4242");
createData->coupon = boost::optional<utility::string_t>(U("WELCOME10"));

api->createSubscription(tenantId, *createData)
.then([](pplx::task<std::shared_ptr<CreateSubscriptionAPIResponse>> task) {
    try {
        auto resp = task.get();
        if (resp) {
            std::cout << "Subscription created: " << resp->subscriptionId << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "Create subscription failed: " << e.what() << std::endl;
    }
});
[inline-code-end]
