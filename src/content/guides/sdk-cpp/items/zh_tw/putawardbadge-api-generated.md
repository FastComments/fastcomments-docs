## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| badgeId | string | 是 |  |
| options | const PutAwardBadgeOptions& | 是 |  |

## 回應

返回：[`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AwardUserBadgeResponse.h)

## 範例

[inline-code-attrs-start title = 'putAwardBadge 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto badgeId = utility::conversions::to_string_t("badge-456");
PutAwardBadgeOptions opts;
opts.userId = utility::conversions::to_string_t("user-42");
opts.note = boost::optional<utility::string_t>(utility::conversions::to_string_t("Excellent comment"));
api->putAwardBadge(tenantId, badgeId, opts).then([](pplx::task<std::shared_ptr<AwardUserBadgeResponse>> t){
    try{
        auto resp = t.get();
        auto respCopy = std::make_shared<AwardUserBadgeResponse>(*resp);
    }catch(const std::exception& e){
    }
});
[inline-code-end]