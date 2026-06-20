## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| setCommentTextParams | SetCommentTextParams | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentTextResponse.h)

## 예제

[inline-code-attrs-start title = 'postSetCommentText 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-9a8b7c6d");
auto paramsPtr = std::make_shared<SetCommentTextParams>();
paramsPtr->text = U("Updated comment: I've rephrased for clarity.");
paramsPtr->editedBy = U("moderator@example.com");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-abc123"));

api->postSetCommentText(commentId, *paramsPtr, sso)
    .then([](pplx::task<std::shared_ptr<SetCommentTextResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) (void)resp;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---