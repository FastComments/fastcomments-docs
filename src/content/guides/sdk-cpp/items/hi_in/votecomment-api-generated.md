## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| urlId | string | हाँ |  |
| broadcastId | string | हाँ |  |
| voteBodyParams | VoteBodyParams | हाँ |  |
| options | const VoteCommentOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/VoteResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'voteComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-7890");
auto urlId = utility::conversions::to_string_t("article-456");
auto broadcastId = utility::conversions::to_string_t("broadcast-321");

VoteBodyParams voteParams;
voteParams.upvote = true;
voteParams.note = boost::optional<utility::string_t>(utility::conversions::to_string_t("Great insight"));

VoteCommentOptions options;
options.dryRun = boost::optional<bool>(false);

api->voteComment(tenantId, commentId, urlId, broadcastId, voteParams, options)
   .then([](pplx::task<std::shared_ptr<VoteResponse>> t) {
       try {
           auto response = t.get();
           // प्रतिक्रिया को संभालें
       } catch (const std::exception&) {
       }
   });
[inline-code-end]