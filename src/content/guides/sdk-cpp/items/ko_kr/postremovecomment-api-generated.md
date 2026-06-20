## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PostRemoveCommentResponse.h)

## 예제

[inline-code-attrs-start title = 'postRemoveComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->postRemoveComment(commentId, sso)
    .then([](pplx::task<std::shared_ptr<PostRemoveCommentResponse>> t) {
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<PostRemoveCommentResponse>();
            if (result) std::cout << "Comment removed successfully\n";
        } catch (const std::exception &e) {
            std::cerr << "Remove failed: " << e.what() << "\n";
        }
    });
[inline-code-end]

---