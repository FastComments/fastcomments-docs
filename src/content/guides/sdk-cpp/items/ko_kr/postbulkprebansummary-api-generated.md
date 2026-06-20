## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| bulkPreBanParams | BulkPreBanParams | 예 |  |
| includeByUserIdAndEmail | bool | 아니요 |  |
| includeByIP | bool | 아니요 |  |
| includeByEmailDomain | bool | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkPreBanSummary.h)

## 예제

[inline-code-attrs-start title = 'postBulkPreBanSummary 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
BulkPreBanParams params;
params.tenantId = utility::string_t(U("my-tenant-123"));
params.emails = std::vector<utility::string_t>{ U("alice@example.com"), U("bob@example.org") };
params.ipAddresses = std::vector<utility::string_t>{ U("203.0.113.45"), U("198.51.100.22") };
boost::optional<bool> includeByUserIdAndEmail(true);
boost::optional<bool> includeByIP(false);
boost::optional<bool> includeByEmailDomain(true);
boost::optional<utility::string_t> sso(utility::string_t(U("sso-token-xyz")));
api->postBulkPreBanSummary(params, includeByUserIdAndEmail, includeByIP, includeByEmailDomain, sso)
.then([](pplx::task<std::shared_ptr<BulkPreBanSummary>> t){
    try {
        auto summary = t.get();
        if (summary) {
            auto summaryCopy = std::make_shared<BulkPreBanSummary>(*summary);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]