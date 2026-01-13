## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| usernameStartsWith | string | いいえ |  |
| mentionGroupIds | vector<string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`SearchUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsers_200_response.h)

## 例

[inline-code-attrs-start title = 'searchUsers の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
utility::string_t urlId = utility::string_t("/posts/2026/new-feature");
boost::optional<utility::string_t> usernameStartsWith = boost::optional<utility::string_t>(utility::string_t("alex"));
boost::optional<std::vector<utility::string_t>> mentionGroupIds{std::vector<utility::string_t>{utility::string_t("ops-team"), utility::string_t("support")}};
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t("saml-idp"));
api->searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso)
    .then([](pplx::task<std::shared_ptr<SearchUsers_200_response>> t){
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<SearchUsers_200_response>();
        } catch (...) {}
    });
[inline-code-end]

---