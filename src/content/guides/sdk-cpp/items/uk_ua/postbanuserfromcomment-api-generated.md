---
## Параметри

| Ім'я | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| options | const PostBanUserFromCommentOptions& | Так |  |

## Відповідь

Повертає: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## Приклад

[inline-code-attrs-start title = 'postBanUserFromComment Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456789");
PostBanUserFromCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("spam"));
options.durationDays = boost::optional<int>(30);

api->postBanUserFromComment(tenantId, commentId, options)
    .then([](std::shared_ptr<BanUserFromCommentResult> result) {
        // обробка результату
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* обробка помилки */ }
    });
[inline-code-end]

---