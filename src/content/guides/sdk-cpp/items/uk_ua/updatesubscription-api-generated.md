## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Так |  |
| userId | string | Ні |  |

## Відповідь

Повертає: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateSubscriptionAPIResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад updateSubscription'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t subscriptionId = U("sub-456");
UpdateAPIUserSubscriptionData updateData{};
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user-789"));
api->updateSubscription(tenantId, subscriptionId, updateData, userId)
.then([](pplx::task<std::shared_ptr<UpdateSubscriptionAPIResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<UpdateSubscriptionAPIResponse>(*resp);
        }
    } catch (const std::exception&) {}
}).wait();
[inline-code-end]

---