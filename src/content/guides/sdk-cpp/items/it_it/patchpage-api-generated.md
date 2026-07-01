## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateAPIPageData | UpdateAPIPageData | Sì |  |

## Risposta

Restituisce: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchPageAPIResponse.h)

## Esempio

[inline-code-attrs-start title = 'patchPage Esempio'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t pageId = U("page-789");
UpdateAPIPageData update;
update.title = boost::optional<utility::string_t>(U("Updated Page Title"));
update.metadata = boost::optional<utility::string_t>(U("{\"author\":\"jane.doe@example.com\"}"));
api->patchPage(tenantId, pageId, update)
    .then([](std::shared_ptr<PatchPageAPIResponse> response) {
        if (response && response->isSuccess) {
            // gestire il successo
        }
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) {}
    });
[inline-code-end]