Попередні коментатори на сторінці, які Наразі НЕ в мережі. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online, щоб відобразити секцію "Учасники".
Курсорна пагінація за commenterName: сервер обходить частковий індекс {tenantId, urlId, commenterName} від afterName вперед через $gt, без витрат $skip.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| afterName | string | Ні |  |
| afterUserId | string | Ні |  |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> afterName = boost::optional<utility::string_t>(U("jane.doe@example.com"));
boost::optional<utility::string_t> afterUserId = boost::optional<utility::string_t>(U("user-789"));
api->getOfflineUsers(tenantId, urlId, afterName, afterUserId).then([](std::shared_ptr<PageUsersOfflineResponse> resp){
    auto result = resp ? resp : std::make_shared<PageUsersOfflineResponse>();
    (void)result;
});
[inline-code-end]

---