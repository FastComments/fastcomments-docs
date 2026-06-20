## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| value | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationPageSearchResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getSearchPages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t value = U("getting-started");
utility::string_t sso = U("user@example.com");
boost::optional<utility::string_t> valueOpt = value;
boost::optional<utility::string_t> ssoOpt = sso;
api->getSearchPages(valueOpt, ssoOpt)
.then([](pplx::task<std::shared_ptr<ModerationPageSearchResponse>> t){
    try {
        auto resp = t.get();
        auto safeResp = resp ? resp : std::make_shared<ModerationPageSearchResponse>();
        (void)safeResp;
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]