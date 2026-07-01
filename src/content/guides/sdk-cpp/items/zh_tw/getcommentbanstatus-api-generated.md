## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

返回: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentBanStatusResponse.h)

## 範例

[inline-code-attrs-start title = 'getCommentBanStatus 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U"my-tenant-123");
auto commentId = utility::string_t(U"comment-456");
boost::optional<utility::string_t> sso = utility::string_t(U"user@example.com");

api->getCommentBanStatus(tenantId, commentId, sso).then([](pplx::task<std::shared_ptr<GetCommentBanStatusResponse>> t){
    try{
        auto response = t.get();
    }catch(const std::exception&){ }
});
[inline-code-end]