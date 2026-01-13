## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |

## Відповідь

Повертає: [`GetVotesForUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesForUser_200_response.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getVotesForUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-987");
boost::optional<utility::string_t> userId = utility::string_t(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;
api->getVotesForUser(tenantId, urlId, userId, anonUserId)
.then([](pplx::task<std::shared_ptr<GetVotesForUser_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copy = std::make_shared<GetVotesForUser_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---