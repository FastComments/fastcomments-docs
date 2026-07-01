## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | CreateCommentParams | Yes |  |
| options | const SaveCommentOptions& | Yes |  |

## Отговор

Връща: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APISaveCommentResponse.h)

## Пример

[inline-code-attrs-start title = 'пример за saveComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateCommentParams commentParams;
commentParams.body = utility::string_t(U("Great article!"));
commentParams.author = utility::string_t(U("jane.doe@example.com"));
commentParams.parentId = boost::optional<utility::string_t>(utility::string_t(U("parent-789")));

SaveCommentOptions options;
options.preview = boost::optional<bool>(false);

api->saveComment(utility::string_t(U("my-tenant-123")), commentParams, options)
    .then([](std::shared_ptr<APISaveCommentResponse> response) {
        auto commentId = response->commentId;
    });
[inline-code-end]