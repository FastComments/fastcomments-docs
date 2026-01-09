## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Yes |  |

## Одговор

Враћа: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateSubscriptionAPIResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример createSubscription'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto payload = std::make_shared<CreateAPIUserSubscriptionData>();
payload->userEmail = U("user@example.com");
payload->planId = U("pro-monthly");
payload->couponCode = boost::optional<utility::string_t>(U("WELCOME10"));
payload->trialDays = boost::optional<int>(14);
payload->autoRenew = boost::optional<bool>(true);
api->createSubscription(tenantId, *payload).then([](std::shared_ptr<CreateSubscriptionAPIResponse> resp) {
    if (resp) std::cout << "Subscription created successfully" << std::endl;
    else std::cout << "Failed to create subscription" << std::endl;
});
[inline-code-end]

---