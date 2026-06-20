## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| commentId | string | Да |  |
| direction | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## Пример

[inline-code-attrs-start title = 'postVote Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> direction = U("up");
boost::optional<utility::string_t> sso = U("sso-token-abc123");
api->postVote(commentId, direction, sso)
.then([](pplx::task<std::shared_ptr<VoteResponse>> task) {
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<VoteResponse>();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---