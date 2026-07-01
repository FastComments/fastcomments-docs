## Parameters

| Naam | Type | Vereist | Omschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| options | const GetGifsTrendingOptions& | Ja |  |

## Respons

Retourneert: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetGifsTrendingResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getGifsTrending Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetGifsTrendingOptions options;
options.limit = 10;
options.rating = boost::optional<utility::string_t>(U("G"));
api->getGifsTrending(tenantId, options).then([](std::shared_ptr<GetGifsTrendingResponse> response) {
    for (const auto& gif : response->gifs) {
        auto url = gif.url;
    }
});
[inline-code-end]