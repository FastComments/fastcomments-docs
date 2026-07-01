## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| id | string | Да |  |
| userId | string | Не |  |

## Одговор

Враћа: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSubscriptionAPIResponse.h)

## Пример

[inline-code-attrs-start title = 'Primer deleteSubscription'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->deleteSubscription(utility::string_t(U("my-tenant-123")), utility::string_t(U("sub-456")), boost::optional<utility::string_t>(utility::string_t(U("user@example.com"))))
    .then([](std::shared_ptr<DeleteSubscriptionAPIResponse> resp){
        if (resp) {
        }
    });
[inline-code-end]