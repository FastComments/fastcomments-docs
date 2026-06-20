## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | はい |  |
| editKey | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPIDeleteCommentResponse.h)

## 例

[inline-code-attrs-start title = 'deleteCommentPublic の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentId = utility::conversions::to_string_t("cmt-456");
utility::string_t broadcastId = utility::conversions::to_string_t("video-789");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(utility::conversions::to_string_t("editkey-xyz"));
boost::optional<utility::string_t> sso = boost::none;
api->deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso)
.then([](pplx::task<std::shared_ptr<PublicAPIDeleteCommentResponse>> t){
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<PublicAPIDeleteCommentResponse>();
        (void)finalResp;
    } catch (const std::exception&) {}
});
[inline-code-end]

---