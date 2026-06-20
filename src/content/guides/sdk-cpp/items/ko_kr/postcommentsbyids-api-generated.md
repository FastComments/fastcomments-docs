## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| commentsByIdsParams | CommentsByIdsParams | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## 예제

[inline-code-attrs-start title = 'postCommentsByIds 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CommentsByIdsParams params;
params.tenantId = utility::string_t(U("my-tenant-123"));
params.commentIds = std::vector<utility::string_t>{ U("cmt-1001"), U("cmt-1002") };
boost::optional<utility::string_t> sso(U("user@example.com"));
api->postCommentsByIds(params, sso).then([](pplx::task<std::shared_ptr<ModerationAPIChildCommentsResponse>> t) {
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<ModerationAPIChildCommentsResponse>();
        std::cout << "Fetched child comments response: " << (result ? "present" : "empty") << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "Request failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---