## Parametri

| Name | Type | Obbligatorio | Descrizione |
|------|------|---------------|-------------|
| tenantId | string | Sì |  |
| postIds | vector<string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FeedPostsStatsResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di getFeedPostsStats'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
std::vector<utility::string_t> postIds = { U("post-1001"), U("post-1002"), U("post-1003") };
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getFeedPostsStats(tenantId, postIds, sso)
    .then([](pplx::task<std::shared_ptr<FeedPostsStatsResponse>> previous) {
        try {
            auto stats = previous.get();
            if (!stats) stats = std::make_shared<FeedPostsStatsResponse>();
            // elabora le statistiche qui (ad esempio, ispeziona i campi, aggiorna l'interfaccia utente)
        } catch (const std::exception&) {
        }
    });
[inline-code-end]