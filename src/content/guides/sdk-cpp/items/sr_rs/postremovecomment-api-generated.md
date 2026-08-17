## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| options | const PostRemoveCommentOptions& | Да |  |

## Одговор

Враћа: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PostRemoveCommentApiResponse.h)

## Пример

[inline-code-attrs-start title = 'postRemoveComment пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("cmt-456789"));
PostRemoveCommentOptions options;
options.permanent = boost::optional<bool>(true);
api->postRemoveComment(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<PostRemoveCommentApiResponse>> task) {
        try {
            auto response = task.get();
            // Обради одговор
        } catch (const std::exception& ex) {
            // Обради грешку
        }
    });
[inline-code-end]