## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| reviewed | bool | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'postSetCommentReviewStatus 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-2026-0001");
boost::optional<bool> reviewed = true;
boost::optional<utility::string_t> sso = U("user@example.com");
auto fallback = std::make_shared<APIEmptyResponse>();
api->postSetCommentReviewStatus(commentId, reviewed, sso)
    .then([fallback](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = fallback;
            std::cout << "Comment review status updated\n";
        } catch (const std::exception &e) {
            std::cerr << "Failed to set review status: " << e.what() << '\n';
        }
    });
[inline-code-end]

---