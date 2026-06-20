## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentTextResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getModerationCommentText'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getModerationCommentText(commentId, sso)
.then([](pplx::task<std::shared_ptr<GetCommentTextResponse>> t) -> std::shared_ptr<GetCommentTextResponse> {
    try {
        auto resp = t.get();
        return resp ? resp : std::make_shared<GetCommentTextResponse>();
    } catch (...) {
        return std::make_shared<GetCommentTextResponse>();
    }
});
[inline-code-end]

---