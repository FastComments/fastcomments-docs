## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| deleteComments | string | いいえ |  |
| commentDeleteMode | string | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 例

[inline-code-attrs-start title = 'deleteTenantUser の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> deleteComments = boost::optional<utility::string_t>(U("true"));
boost::optional<utility::string_t> commentDeleteMode = boost::optional<utility::string_t>(U("soft"));
auto fallback = std::make_shared<FlagCommentPublic_200_response>();
api->deleteTenantUser(tenantId, userId, deleteComments, commentDeleteMode)
.then([fallback](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = fallback;
        std::cout << "deleteTenantUser completed\n";
    } catch (const std::exception &e) {
        std::cout << "deleteTenantUser failed: " << e.what() << "\n";
    }
}).wait();
[inline-code-end]

---