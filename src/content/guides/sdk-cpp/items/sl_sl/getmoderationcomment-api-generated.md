---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const GetModerationCommentOptions& | Yes |  |

## Odgovor

Vrne: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPICommentResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getModerationComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-456");
GetModerationCommentOptions options;
options.includeDeleted = boost::optional<bool>(true);
options.locale = boost::optional<utility::string_t>(utility::conversions::to_string_t("en-US"));
api->getModerationComment(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<ModerationAPICommentResponse>> task) {
        try {
            auto response = task.get();
            // Obdelajte odgovor po potrebi
        } catch (const std::exception& ex) {
            // Obvladujte napako
        }
    });
[inline-code-end]

---