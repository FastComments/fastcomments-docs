## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| email | string | Tak |  |

## Odpowiedź

Zwraca: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByEmailAPIResponse.h)

## Przykład

[inline-code-attrs-start title = 'getSSOUserByEmail Przykład'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto correlationId = boost::optional<utility::string_t>(utility::conversions::to_string_t("corr-001"));

api->getSSOUserByEmail(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("user@example.com")
).then([](pplx::task<std::shared_ptr<GetSSOUserByEmailAPIResponse>> t) {
    try {
        auto response = t.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]