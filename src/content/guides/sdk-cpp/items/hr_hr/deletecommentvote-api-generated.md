## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| voteId | string | Da |  |
| urlId | string | Da |  |
| broadcastId | string | Da |  |
| options | const DeleteCommentVoteOptions& | Da |  |

## Odgovor

Vraća: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## Primjer

[inline-code-attrs-start title = 'deleteCommentVote Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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