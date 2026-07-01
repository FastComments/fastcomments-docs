## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## 回應

返回：[`GetVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesResponse.h)

## 範例

[inline-code-attrs-start title = 'getVotes 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> extraHeader = boost::none;

api->getVotes(tenantId, urlId).then([=](pplx::task<std::shared_ptr<GetVotesResponse>> task) {
    try {
        auto original = task.get();
        auto response = std::make_shared<GetVotesResponse>(*original);
    } catch (...) {
        // 錯誤處理
    }
});
[inline-code-end]