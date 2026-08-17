---
## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| options | const UnFlagCommentOptions& | Yes |  |

## 응답

반환: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## 예시

[inline-code-attrs-start title = 'unFlagComment 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = UnFlagCommentOptions{};
options.reason = boost::optional<utility::string_t>(U("Resolved by moderator"));
api->unFlagComment(U("my-tenant-123"), U("comment-456"), options)
    .then([](std::shared_ptr<FlagCommentResponse> response) {
        if (response) {
            auto status = response->status;
            // 필요에 따라 상태를 처리합니다
        }
    })
    .then([](pplx::task<void> previous) {
        try {
            previous.get();
        } catch (const std::exception& e) {
            // 오류를 처리합니다
        }
    });
[inline-code-end]

---