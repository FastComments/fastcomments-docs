## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| userId | string | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserTrustFactorResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer getTrustFactor'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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