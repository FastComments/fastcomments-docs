## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetLogsResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getLogs'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getLogs(commentId, sso)
.then([](std::shared_ptr<ModerationAPIGetLogsResponse> resp){
    if (!resp) return;
    auto localCopy = std::make_shared<ModerationAPIGetLogsResponse>(*resp);
}).wait();
[inline-code-end]

---