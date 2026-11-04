Включение или отключение уведомлений для страницы. Когда пользователи подписаны на страницу, уведомления создаются
для новых корневых комментариев, а также

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| url | string | Да |  |
| pageTitle | string | Да |  |
| subscribedOrUnsubscribed | string | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/UpdateUserNotificationPageSubscriptionStatusResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример updateUserNotificationPageSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
utility::string_t url = U("https://example.com/articles/awesome-article");
utility::string_t pageTitle = U("Awesome Article");
utility::string_t subscription = U("subscribed");
boost::optional<utility::string_t> sso = boost::make_optional<utility::string_t>(U("sso-token-789"));

api->updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscription, sso)
    .then([](std::shared_ptr<UpdateUserNotificationPageSubscriptionStatusResponse> resp) {
        // обработать resp при необходимости
    });
[inline-code-end]