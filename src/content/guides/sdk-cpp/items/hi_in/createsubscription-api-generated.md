## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateSubscriptionAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'createSubscription उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
CreateAPIUserSubscriptionData subscriptionData;
subscriptionData.email = U("user@example.com");
subscriptionData.planId = U("premium-plan");
subscriptionData.couponCode = boost::optional<utility::string_t>(U("WELCOME10"));
api->createSubscription(tenantId, subscriptionData)
    .then([](pplx::task<std::shared_ptr<CreateSubscriptionAPIResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]