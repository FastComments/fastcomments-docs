## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentIds | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CheckBlockedCommentsResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'checkedCommentsForBlocked Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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