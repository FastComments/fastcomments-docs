## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Odgovor

Vrne: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV2PageReacts.h)

## Primer

[inline-code-attrs-start title = 'Primer getV2PageReacts'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t("my-tenant-123");
auto urlId = utility::string_t("https://example.com/article/42");
boost::optional<utility::string_t> locale = utility::string_t("en-US");
api->getV2PageReacts(tenantId, urlId)
    .then([locale](std::shared_ptr<GetV2PageReacts> reacts){
        if (locale) {}
    })
    .then([](pplx::task<void> t){
        try { t.get(); } catch (...) {}
    });
[inline-code-end]