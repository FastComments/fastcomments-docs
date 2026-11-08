## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| broadcastId | string | हाँ |  |
| commentData | CommentData | हाँ |  |
| options | const CreateCommentPublicOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/SaveCommentsResponseWithPresence.h)

## उदाहरण

[inline-code-attrs-start title = 'createCommentPublic उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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