## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| includeEmail | bool | 아니요 |  |
| includeIP | bool | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPICommentResponse.h)

## 예제

[inline-code-attrs-start title = 'getModerationComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654");
boost::optional<bool> includeEmail = boost::optional<bool>(true);
boost::optional<bool> includeIP = boost::optional<bool>(false);
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->getModerationComment(commentId, includeEmail, includeIP, sso)
.then([](std::shared_ptr<ModerationAPICommentResponse> resp) {
    auto result = resp ? resp : std::make_shared<ModerationAPICommentResponse>();
    if (resp)
        std::cout << "Comment fetched successfully" << std::endl;
    else
        std::cout << "No comment returned; using placeholder" << std::endl;
}).wait();
[inline-code-end]

---