## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| userId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserTrustFactorResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getTrustFactor Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> userId{ U("user@example.com") };
boost::optional<utility::string_t> sso{ U("my-tenant-123") };
api->getTrustFactor(userId, sso)
    .then([](std::shared_ptr<GetUserTrustFactorResponse> resp) {
        if (resp) {
            auto tag = std::make_shared<utility::string_t>(U("trust-check"));
            (void)tag;
        }
    });
[inline-code-end]

---