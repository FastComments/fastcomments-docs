## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostBanUserFromCommentOptions& | Yes |  |

## Одговор

Враћа: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## Пример

[inline-code-attrs-start title = 'postBanUserFromComment Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456789");
PostBanUserFromCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("spam"));
options.durationDays = boost::optional<int>(30);

api->postBanUserFromComment(tenantId, commentId, options)
    .then([](std::shared_ptr<BanUserFromCommentResult> result) {
        // обрада резултата
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* обрада грешке */ }
    });
[inline-code-end]