---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 回應

回傳: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesResponse.h)

## 範例

[inline-code-attrs-start title = 'getVotes 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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