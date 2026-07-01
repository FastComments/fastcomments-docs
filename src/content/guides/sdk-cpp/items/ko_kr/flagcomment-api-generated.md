## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| options | const FlagCommentOptions& | 예 |  |

## 응답

반환: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## 예제

[inline-code-attrs-start title = 'flagComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<FlagCommentOptions>();
opts->reason = utility::conversions::to_string_t("spam");
opts->note = boost::optional<utility::string_t>(utility::conversions::to_string_t("User posted duplicate links"));

api->flagComment(utility::conversions::to_string_t("my-tenant-123"),
                 utility::conversions::to_string_t("comment-456"),
                 *opts)
    .then([](pplx::task<std::shared_ptr<FlagCommentResponse>> t) {
        auto resp = t.get();
    });
[inline-code-end]