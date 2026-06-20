## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'postUnFlagComment 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-8f3a21b7");
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));
api->postUnFlagComment(commentId, sso)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<APIEmptyResponse>();
    } catch (const std::exception &e) {
    }
}).wait();
[inline-code-end]

---