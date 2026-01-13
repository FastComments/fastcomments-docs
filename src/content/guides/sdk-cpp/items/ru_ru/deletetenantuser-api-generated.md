## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| deleteComments | string | Нет |  |
| commentDeleteMode | string | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример deleteTenantUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> deleteComments = boost::optional<utility::string_t>(U("true"));
boost::optional<utility::string_t> commentDeleteMode = boost::optional<utility::string_t>(U("soft"));
auto fallback = std::make_shared<FlagCommentPublic_200_response>();
api->deleteTenantUser(tenantId, userId, deleteComments, commentDeleteMode)
.then([fallback](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = fallback;
        std::cout << "deleteTenantUser completed\n";
    } catch (const std::exception &e) {
        std::cout << "deleteTenantUser failed: " << e.what() << "\n";
    }
}).wait();
[inline-code-end]

---