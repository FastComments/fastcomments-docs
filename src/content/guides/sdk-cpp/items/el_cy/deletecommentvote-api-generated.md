## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| voteId | string | Ναι |  |
| urlId | string | Ναι |  |
| broadcastId | string | Ναι |  |
| editKey | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteCommentVote'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
utility::string_t voteId = U("vote-abc123");
utility::string_t urlId = U("https://example.com/articles/42");
utility::string_t broadcastId = U("bcast-001");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("edit-key-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, editKey, sso)
.then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> t){
    try {
        auto respPtr = t.get();
        std::shared_ptr<VoteDeleteResponse> result = respPtr ? std::make_shared<VoteDeleteResponse>(*respPtr) : std::make_shared<VoteDeleteResponse>();
        (void)result;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---