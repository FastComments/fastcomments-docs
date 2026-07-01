## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostVoteOptions& | Yes |  |

## Απόκριση

Επιστρέφει: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postVote'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
PostVoteOptions options;
options.upvote = boost::make_optional(true);
options.reason = boost::make_optional<std::string>("Inappropriate content");
api->postVote(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<VoteResponse>> t) {
        try {
            auto resp = t.get();
            auto count = resp->voteCount;
        } catch (const std::exception& e) {
        }
    });
[inline-code-end]