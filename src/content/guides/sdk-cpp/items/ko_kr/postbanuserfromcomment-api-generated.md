## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostBanUserFromCommentOptions& | Yes |  |

## 응답

반환: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## 예제

[inline-code-attrs-start title = 'postBanUserFromComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456789");
PostBanUserFromCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("spam"));
options.durationDays = boost::optional<int>(30);

api->postBanUserFromComment(tenantId, commentId, options)
    .then([](std::shared_ptr<BanUserFromCommentResult> result) {
        // 결과 처리
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* 오류 처리 */ }
    });
[inline-code-end]