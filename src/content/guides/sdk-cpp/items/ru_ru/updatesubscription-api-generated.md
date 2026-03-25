## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Да |  |
| userId | string | Нет |  |

## Ответ

Возвращает: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateSubscriptionAPIResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример updateSubscription'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t subscriptionId = U("sub-456");
UpdateAPIUserSubscriptionData updateData;
auto fallbackResp = std::make_shared<UpdateSubscriptionAPIResponse>();
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
api->updateSubscription(tenantId, subscriptionId, updateData, userId)
    .then([](pplx::task<std::shared_ptr<UpdateSubscriptionAPIResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) {
                // обработать resp
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---