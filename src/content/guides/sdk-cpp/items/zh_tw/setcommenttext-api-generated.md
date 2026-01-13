## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`SetCommentText_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentText_200_response.h)

## 範例

[inline-code-attrs-start title = 'setCommentText 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---