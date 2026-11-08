## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| voteId | string | हाँ |  |
| urlId | string | हाँ |  |
| broadcastId | string | हाँ |  |
| options | const DeleteCommentVoteOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteCommentVote उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = DeleteCommentVoteOptions{};
options.reason = boost::optional<utility::string_t>(U("spam"));
options.adminOverride = boost::optional<bool>(true);

api->deleteCommentVote(
    U("my-tenant-123"),
    U("cmt-456789"),
    U("vote-987"),
    U("url-abc123"),
    U("broadcast-001"),
    options
).then([](std::shared_ptr<VoteDeleteResponse> resp){
    if (resp) {
        std::cout << "Deleted vote, code: " << resp->code << std::endl;
    }
});
[inline-code-end]