## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| voteId | string | Tak |  |
| urlId | string | Tak |  |
| broadcastId | string | Tak |  |
| editKey | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`DeleteCommentVote_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentVote_200_response.h)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteCommentVote'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-789");
utility::string_t voteId = U("vote-456");
utility::string_t urlId = U("/articles/2026/fastcomments-guide");
utility::string_t broadcastId = U("broadcast-001");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("editkey-abc"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto placeholder = std::make_shared<DeleteCommentVote_200_response>();
api->deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, editKey, sso)
.then([=](pplx::task<std::shared_ptr<DeleteCommentVote_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Vote deleted\n";
    } catch (const std::exception &e) {
        std::cerr << "Delete failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---