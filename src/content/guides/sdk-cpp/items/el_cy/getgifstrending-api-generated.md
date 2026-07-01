## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetGifsTrendingOptions& | Yes |  |

## Απόκριση

Επιστρέφει: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetGifsTrendingResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getGifsTrending'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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