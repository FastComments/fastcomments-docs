## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesResponse.h)

## 예제

[inline-code-attrs-start title = 'getVotes 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<int> limit = 50;
auto fallback = std::make_shared<GetVotesResponse>();
api->getVotes(utility::conversions::to_string_t("my-tenant-123"), utility::conversions::to_string_t("article-9876"))
.then([fallback, limit](pplx::task<std::shared_ptr<GetVotesResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = fallback;
        if (limit) {
            auto processed = std::make_shared<GetVotesResponse>(*resp);
        }
    } catch (const std::exception& e) {
        auto errorResp = std::make_shared<GetVotesResponse>();
    }
});
[inline-code-end]

---