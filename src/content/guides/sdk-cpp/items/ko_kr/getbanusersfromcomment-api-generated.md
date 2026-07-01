## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetBannedUsersFromCommentResponse.h)

## 예제

[inline-code-attrs-start title = 'getBanUsersFromComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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