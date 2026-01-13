## Параметры

| Name | Type | Обязательно | Описание |
|------|------|------------|-------------|
| tenantId | string | Да |  |
| userId | string | Нет |  |
| urlId | string | Нет |  |
| fromCommentId | string | Нет |  |
| viewed | bool | Нет |  |
| type | string | Нет |  |

## Ответ

Возвращает: [`GetNotificationCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCount_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример getNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = U("user@example.com");
boost::optional<utility::string_t> urlId = U("https://www.example.com/article/456");
boost::optional<utility::string_t> fromCommentId = U("cmt-789");
boost::optional<bool> viewed = true;
boost::optional<utility::string_t> type = U("reply");

api->getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, type)
    .then([](pplx::task<std::shared_ptr<GetNotificationCount_200_response>> task){
        try {
            auto resp = task.get();
            auto result = resp ? resp : std::make_shared<GetNotificationCount_200_response>();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---