## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| commentsByIdsParams | CommentsByIdsParams | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад postCommentsByIds'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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