## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentResponse.h)

## 예제

[inline-code-attrs-start title = 'getComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> include = boost::optional<utility::string_t>(U("author,replies"));

api->getComment(tenantId, commentId).then([include](std::shared_ptr<APIGetCommentResponse> resp) {
    if (resp) {
        auto copy = std::make_shared<APIGetCommentResponse>(*resp);
        std::cout << "Comment retrieved for tenant" << std::endl;
    } else {
        std::cout << "Comment not found" << std::endl;
    }
});
[inline-code-end]

---