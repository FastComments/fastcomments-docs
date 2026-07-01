## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const GetModerationCommentOptions& | Yes |  |

## Odpowiedź

Returns: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPICommentResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getModerationComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // Przetwórz odpowiedź w razie potrzeby
        } catch (const std::exception& ex) {
            // Obsłuż błąd
        }
    });
[inline-code-end]