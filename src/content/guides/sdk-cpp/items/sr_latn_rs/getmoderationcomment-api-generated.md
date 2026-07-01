## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| options | const GetModerationCommentOptions& | Da |  |

## Odgovor

Vraća: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPICommentResponse.h)

## Primer

[inline-code-attrs-start title = 'getModerationComment Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // Obradi odgovor po potrebi
        } catch (const std::exception& ex) {
            // Obradi grešku
        }
    });
[inline-code-end]