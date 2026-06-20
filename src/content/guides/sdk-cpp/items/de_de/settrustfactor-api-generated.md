---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| userId | string | Nein |  |
| trustFactor | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetUserTrustFactorResponse.h)

## Beispiel

[inline-code-attrs-start title = 'setTrustFactor Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> userId = utility::conversions::to_string_t("user-9876");
boost::optional<utility::string_t> trustFactor = utility::conversions::to_string_t("verified");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("sso-token-abc123");
api->setTrustFactor(userId, trustFactor, sso)
.then([](pplx::task<std::shared_ptr<SetUserTrustFactorResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<SetUserTrustFactorResponse>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---