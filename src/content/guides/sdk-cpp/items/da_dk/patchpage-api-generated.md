## Parametre

| Navn | Type | Krævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateAPIPageData | UpdateAPIPageData | Ja |  |

## Svar

Returnerer: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchPageAPIResponse.h)

## Eksempel

[inline-code-attrs-start title = 'patchPage Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t pageId = U("page-789");
UpdateAPIPageData update;
update.title = boost::optional<utility::string_t>(U("Updated Page Title"));
update.metadata = boost::optional<utility::string_t>(U("{\"author\":\"jane.doe@example.com\"}"));
api->patchPage(tenantId, pageId, update)
    .then([](std::shared_ptr<PatchPageAPIResponse> response) {
        if (response && response->isSuccess) {
            // håndter succes
        }
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) {}
    });
[inline-code-end]