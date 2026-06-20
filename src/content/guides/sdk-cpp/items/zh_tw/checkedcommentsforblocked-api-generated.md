## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| commentIds | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CheckBlockedCommentsResponse.h)

## 範例

[inline-code-attrs-start title = 'checkedCommentsForBlocked 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentIds = utility::conversions::to_string_t("cmt-456,cmt-789");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::conversions::to_string_t("user@example.com"));

api->checkedCommentsForBlocked(tenantId, commentIds, sso)
    .then([](pplx::task<std::shared_ptr<CheckBlockedCommentsResponse>> t) {
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<CheckBlockedCommentsResponse>();
            (void)result;
        } catch (const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]

---