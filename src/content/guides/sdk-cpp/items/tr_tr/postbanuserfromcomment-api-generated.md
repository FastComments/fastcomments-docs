## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|----------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostBanUserFromCommentOptions& | Yes |  |

## Yanıt

Döndürür: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## Örnek

[inline-code-attrs-start title = 'postBanUserFromComment Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456789");
PostBanUserFromCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("spam"));
options.durationDays = boost::optional<int>(30);

api->postBanUserFromComment(tenantId, commentId, options)
    .then([](std::shared_ptr<BanUserFromCommentResult> result) {
        // sonuç işleme
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* hata işleme */ }
    });
[inline-code-end]