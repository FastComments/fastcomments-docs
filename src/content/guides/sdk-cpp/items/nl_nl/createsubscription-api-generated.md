## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Ja |  |

## Response

Retourneert: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateSubscriptionAPIResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'createSubscription Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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