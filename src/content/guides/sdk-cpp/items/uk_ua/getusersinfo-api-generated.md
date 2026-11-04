Масова інформація про користувачів для орендаря. За заданими userIds повертає інформацію для відображення з User / SSOUser.
Використовується віджетом коментарів для збагачення користувачів, які щойно з'явилися через подію присутності.
Без контексту сторінки: приватність застосовується однаково (приватні профілі маскуються).

## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| ids | string | Так |  |

## Відповідь

Повертає: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // обробка відповіді
    }catch(const std::exception&){
        // обробка помилки
    }
});
[inline-code-end]