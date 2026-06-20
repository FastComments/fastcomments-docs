## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| search | string | Ja |  |
| locale | string | Nee |  |
| rating | string | Nee |  |
| page | double | Nee |  |

## Antwoord

Retourneert: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetGifsSearchResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getGifsSearch Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t search = U("funny cats");
boost::optional<utility::string_t> locale(U("en-US"));
boost::optional<utility::string_t> rating(U("pg"));
boost::optional<double> page(1.0);

api->getGifsSearch(tenantId, search, locale, rating, page)
.then([](pplx::task<std::shared_ptr<GetGifsSearchResponse>> t) {
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<GetGifsSearchResponse>();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---