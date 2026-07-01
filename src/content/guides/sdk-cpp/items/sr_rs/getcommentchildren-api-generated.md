## Параметри

| Име | Тип | Неопходно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| sso | string | Не |  |

## Одговор

Враћа: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## Пример

[inline-code-attrs-start title = 'getCommentChildren Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
boost::optional<utility::string_t> sss = U("sso-token-abc");

api->getCommentChildren(tenantId, commentId, sss)
    .then([](pplx::task<std::shared_ptr<ModerationAPIChildCommentsResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]