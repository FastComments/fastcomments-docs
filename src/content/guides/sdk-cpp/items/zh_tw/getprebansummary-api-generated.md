## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| includeByUserIdAndEmail | bool | 否 |  |
| includeByIP | bool | 否 |  |
| includeByEmailDomain | bool | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`PreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PreBanSummary.h)

## 範例

[inline-code-attrs-start title = 'getPreBanSummary 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = utility::conversions::to_string_t("cmt-987654321");
boost::optional<bool> includeByUserIdAndEmail = true;
boost::optional<bool> includeByIP = true;
boost::optional<bool> includeByEmailDomain = false;
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("my-tenant-123-sso-token");
api->getPreBanSummary(commentId, includeByUserIdAndEmail, includeByIP, includeByEmailDomain, sso)
    .then([](pplx::task<std::shared_ptr<PreBanSummary>> t) {
        try {
            auto summary = t.get();
            if (!summary) summary = std::make_shared<PreBanSummary>();
            std::cout << "PreBanSummary retrieved" << std::endl;
        } catch (const std::exception &e) {
            std::cerr << "Error fetching PreBanSummary: " << e.what() << std::endl;
        }
    });
[inline-code-end]

---