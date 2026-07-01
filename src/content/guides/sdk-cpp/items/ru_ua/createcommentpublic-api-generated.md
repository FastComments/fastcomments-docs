## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentData | CommentData | Yes |  |
| options | const CreateCommentPublicOptions& | Yes |  |

## Відповідь

Повертає: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SaveCommentsResponseWithPresence.h)

## Приклад

[inline-code-attrs-start title = 'createCommentPublic Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto comment = CommentData();
comment.body = U("This is a test comment");
comment.authorName = U("John Doe");
comment.authorEmail = U("john.doe@example.com");

CreateCommentPublicOptions opts;
opts.replyToCommentId = boost::optional<utility::string_t>(U("parent-123"));

api->createCommentPublic(
    U("my-tenant-123"),
    U("article-456"),
    U("broadcast-789"),
    comment,
    opts).then([](std::shared_ptr<SaveCommentsResponseWithPresence> res){
        auto commentId = res->commentId;
    });
[inline-code-end]