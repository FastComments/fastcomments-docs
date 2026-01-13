## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| usernameStartsWith | string | לא |  |
| mentionGroupIds | vector<string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`SearchUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsers_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-searchUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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