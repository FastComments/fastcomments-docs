## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| textSearch | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSuggestResponse.h)

## Primer

[inline-code-attrs-start title = 'getSearchSuggest Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> textSearch = utility::string_t(U("preventing spam in comments"));
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));
api->getSearchSuggest(textSearch, sso)
    .then([](pplx::task<std::shared_ptr<ModerationSuggestResponse>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                auto copy = std::make_shared<ModerationSuggestResponse>(*resp);
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---