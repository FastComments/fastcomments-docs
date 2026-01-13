## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`BlockFromCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockFromCommentPublic_200_response.h)

## 範例

[inline-code-attrs-start title = 'blockFromCommentPublic 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
PublicBlockFromCommentParams params;
params.reason = U("Repeated harassment");
params.durationMinutes = 1440;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-abc123"));
auto placeholder = std::make_shared<BlockFromCommentPublic_200_response>();
api->blockFromCommentPublic(tenantId, commentId, params, sso)
.then([](pplx::task<std::shared_ptr<BlockFromCommentPublic_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) std::wcout << U("Comment blocked successfully\n");
        else std::wcout << U("Block request returned empty response\n");
    } catch (...) {
        std::wcout << U("Block request failed\n");
    }
});
[inline-code-end]

---