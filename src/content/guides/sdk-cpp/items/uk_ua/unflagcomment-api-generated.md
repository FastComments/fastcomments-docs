## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |

## Відповідь

Повертає: [`FlagComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagComment_200_response.h)

## Приклад

[inline-code-attrs-start title = 'Приклад unFlagComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;
auto fallback = std::make_shared<FlagComment_200_response>();
api->unFlagComment(tenantId, commentId, userId, anonUserId)
    .then([fallback](pplx::task<std::shared_ptr<FlagComment_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = fallback;
        } catch (...) {
        }
    });
[inline-code-end]

---