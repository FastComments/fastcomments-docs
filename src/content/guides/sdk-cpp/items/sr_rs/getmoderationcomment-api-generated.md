## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const GetModerationCommentOptions& | Yes |  |

## Одговор

Враћа: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationAPICommentResponse.h)

## Пример

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
            // Обради одговор по потреби
        } catch (const std::exception& ex) {
            // Обради грешку
        }
    });
[inline-code-end]