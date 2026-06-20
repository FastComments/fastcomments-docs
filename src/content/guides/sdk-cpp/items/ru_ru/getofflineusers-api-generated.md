Прошлые комментаторы на странице, которые НЕ в сети в данный момент. Отсортированы по displayName.
Используйте это после исчерпания /users/online для отображения раздела «Members».
Курсорная пагинация по commenterName: сервер проходит частичный индекс {tenantId, urlId, commenterName} от afterName вперёд через $gt, без стоимости $skip.

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример использования getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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