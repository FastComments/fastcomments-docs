## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| redirectURL | string | Не |  |

## Одговор

Враћа: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример за sendLoginLink'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> redirectURL = utility::string_t(U("https://app.example.com/welcome"));
api->sendLoginLink(tenantId, userId, redirectURL)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t)
    {
        try
        {
            auto resp = t.get();
            if (resp) return resp;
            return std::make_shared<FlagCommentPublic_200_response>();
        }
        catch (...)
        {
            return std::make_shared<FlagCommentPublic_200_response>();
        }
    });
[inline-code-end]

---