## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| urlId | string | Ναι |  |
| broadcastId | string | Ναι |  |
| voteBodyParams | VoteBodyParams | Ναι |  |
| options | const VoteCommentOptions& | Ναι |  |

## Απόκριση

Επιστρέφει: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'voteComment Παράδειγμα'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
           // handle response
       } catch (const std::exception&) {
       }
   });
[inline-code-end]