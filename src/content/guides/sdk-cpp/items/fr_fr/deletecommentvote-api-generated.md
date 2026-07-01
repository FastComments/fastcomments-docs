## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| voteId | string | Oui |  |
| urlId | string | Oui |  |
| broadcastId | string | Oui |  |
| options | const DeleteCommentVoteOptions& | Oui |  |

## Réponse

Retourne : [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple deleteCommentVote'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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