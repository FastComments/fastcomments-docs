## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| options | const PostVoteOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/VoteResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'postVote उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
PostVoteOptions options;
options.upvote = boost::make_optional(true);
options.reason = boost::make_optional<std::string>("अनुचित सामग्री");
api->postVote(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<VoteResponse>> t) {
        try {
            auto resp = t.get();
            auto count = resp->voteCount;
        } catch (const std::exception& e) {
        }
    });
[inline-code-end]