## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| broadcastId | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeCommentPinStatusResponse.h)

## Приклад

[inline-code-attrs-start title = 'pinComment Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
utility::string_t broadcastId = U("broadcast-789");
boost::optional<utility::string_t> sso = U("sso-token-abc");

api->pinComment(tenantId, commentId, broadcastId, sso)
    .then([](pplx::task<std::shared_ptr<ChangeCommentPinStatusResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---