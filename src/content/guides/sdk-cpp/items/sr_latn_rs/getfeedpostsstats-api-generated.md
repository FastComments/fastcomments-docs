## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postIds | vector<string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FeedPostsStatsResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getFeedPostsStats'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
std::vector<utility::string_t> postIds = { U("post-1001"), U("post-1002"), U("post-1003") };
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getFeedPostsStats(tenantId, postIds, sso)
    .then([](pplx::task<std::shared_ptr<FeedPostsStatsResponse>> previous) {
        try {
            auto stats = previous.get();
            if (!stats) stats = std::make_shared<FeedPostsStatsResponse>();
            // ovde obradite statistiku (npr., pregled polja, ažuriranje UI)
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---