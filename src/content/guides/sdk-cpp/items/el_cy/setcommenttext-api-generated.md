## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| broadcastId | string | Ναι |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Ναι |  |
| editKey | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`SetCommentText_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentText_200_response.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα setCommentText'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456");
utility::string_t broadcastId = U("bcast-789");
CommentTextUpdateRequest commentTextUpdateRequest;
commentTextUpdateRequest.setText(U("Updated comment text by moderator."));
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("edit-key-abc123"));
boost::optional<utility::string_t> sso;
api->setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso)
.then([](pplx::task<std::shared_ptr<SetCommentText_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto result = std::make_shared<SetCommentText_200_response>(*resp);
            std::cout << "Comment updated\n";
        } else {
            std::cout << "No response\n";
        }
    } catch (const std::exception &e) {
        std::cerr << "setCommentText error: " << e.what() << '\n';
    }
});
[inline-code-end]