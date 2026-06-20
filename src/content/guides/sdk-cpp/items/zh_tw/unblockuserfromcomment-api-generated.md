## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnblockSuccess.h)

## 範例

[inline-code-attrs-start title = 'unBlockUserFromComment 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
UnBlockFromCommentParams params;
boost::optional<utility::string_t> userId = utility::string_t(U("user@example.com"));
boost::optional<utility::string_t> anonUserId = utility::string_t(U("anon-98765"));
auto unblockTask = api->unBlockUserFromComment(tenantId, commentId, params, userId, anonUserId)
    .then([](pplx::task<std::shared_ptr<UnblockSuccess>> t) -> std::shared_ptr<UnblockSuccess> {
        try {
            auto result = t.get();
            return result ? result : std::make_shared<UnblockSuccess>();
        } catch (...) {
            return std::make_shared<UnblockSuccess>();
        }
    });
[inline-code-end]

---