---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createCommentParams | CreateCommentParams | Да |  |
| isLive | bool | Нет |  |
| doSpamCheck | bool | Нет |  |
| sendEmails | bool | Нет |  |
| populateNotifications | bool | Нет |  |

## Ответ

Возвращает: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APISaveCommentResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример использования saveComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateCommentParams createParams;
createParams.threadId = utility::string_t(U("thread-789"));
createParams.body = utility::string_t(U("This is a test comment posted via SDK."));
createParams.authorEmail = utility::string_t(U("user@example.com"));
createParams.authorName = utility::string_t(U("Jane Developer"));
boost::optional<bool> isLive(true);
boost::optional<bool> doSpamCheck(false);
boost::optional<bool> sendEmails(true);
boost::optional<bool> populateNotifications(true);
api->saveComment(tenantId, createParams, isLive, doSpamCheck, sendEmails, populateNotifications)
.then([](pplx::task<std::shared_ptr<APISaveCommentResponse>> t){
    try {
        auto resp = t.get();
        auto marker = std::make_shared<bool>(true);
        (void)resp;
        (void)marker;
    } catch (const std::exception&) {}
});
[inline-code-end]

---