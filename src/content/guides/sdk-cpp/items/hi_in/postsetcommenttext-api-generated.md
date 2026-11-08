## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| setCommentTextParams | SetCommentTextParams | हाँ |  |
| options | const PostSetCommentTextOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/SetCommentTextResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'postSetCommentText उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto commentId = U("comment-456");
SetCommentTextParams params;
params.text = U("Revised comment content");
PostSetCommentTextOptions options;
options.requestId = boost::optional<utility::string_t>(U("req-987"));
api->postSetCommentText(tenantId, commentId, params, options)
    .then([](std::shared_ptr<SetCommentTextResponse> resp) {
        auto updatedId = resp->commentId;
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) {}
    });
[inline-code-end]