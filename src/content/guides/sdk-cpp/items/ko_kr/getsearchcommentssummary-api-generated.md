## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| value | string | 아니요 |  |
| filters | string | 아니요 |  |
| searchFilters | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationCommentSearchResponse.h)

## 예제

[inline-code-attrs-start title = 'getSearchCommentsSummary 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> value = boost::optional<utility::string_t>(U("hate speech"));
boost::optional<utility::string_t> filters = boost::optional<utility::string_t>(U("tenantId:my-tenant-123;moderationStatus:unreviewed"));
boost::optional<utility::string_t> searchFilters = boost::optional<utility::string_t>(U("authorEmail:moderator@example.com"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("my-sso-jwt-token-abc123"));
api->getSearchCommentsSummary(value, filters, searchFilters, sso)
    .then([](std::shared_ptr<ModerationCommentSearchResponse> resp){
        auto result = resp ? resp : std::make_shared<ModerationCommentSearchResponse>();
        (void)result;
    });
[inline-code-end]

---