## 參數

| 名稱 | 類型 | 必須 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

返回: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetBannedUsersFromCommentResponse.h)

## 範例

[inline-code-attrs-start title = 'getBanUsersFromComment 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("comment-456"));
boost::optional<utility::string_t> sso = boost::make_optional(utility::string_t(U("sso-token-abc")));

api->getBanUsersFromComment(tenantId, commentId, sso).then([](pplx::task<std::shared_ptr<GetBannedUsersFromCommentResponse>> task) {
    try {
        auto response = task.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]