## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | 예 |  |

## 응답

반환: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateSubscriptionAPIResponse.h)

## 예시

[inline-code-attrs-start title = 'createSubscription 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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