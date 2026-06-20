## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPISetCommentTextResponse.h)

## 範例

[inline-code-attrs-start title = 'setCommentText 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
utility::string_t broadcastId = U("broadcast-789");
CommentTextUpdateRequest commentTextUpdateRequest;
commentTextUpdateRequest.text = U("Updated comment: clarified wording and fixed a typo.");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("editKey-abc123"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso)
    .then([](pplx::task<std::shared_ptr<PublicAPISetCommentTextResponse>> task) {
        try {
            auto resp = task.get();
            if (!resp) resp = std::make_shared<PublicAPISetCommentTextResponse>();
        } catch (const std::exception&) {}
    });
[inline-code-end]

---