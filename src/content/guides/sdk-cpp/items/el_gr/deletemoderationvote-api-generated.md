## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voteId | string | Yes |  |
| options | const DeleteModerationVoteOptions& | Yes |  |

## Απάντηση

Επιστρέφει: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteModerationVote'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-abc123");
auto voteId = utility::conversions::to_string_t("vote-xyz789");
DeleteModerationVoteOptions opts;
opts.reason = utility::conversions::to_string_t("Offensive language");
opts.hardDelete = boost::optional<bool>(true);
api->deleteModerationVote(tenantId, commentId, voteId, opts)
    .then([](std::shared_ptr<VoteDeleteResponse> resp){});
[inline-code-end]