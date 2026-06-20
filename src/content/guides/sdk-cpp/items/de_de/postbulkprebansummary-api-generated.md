## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| bulkPreBanParams | BulkPreBanParams | Ja |  |
| includeByUserIdAndEmail | bool | Nein |  |
| includeByIP | bool | Nein |  |
| includeByEmailDomain | bool | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkPreBanSummary.h)

## Beispiel

[inline-code-attrs-start title = 'postBulkPreBanSummary Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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