## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| options | const GetSearchPagesOptions& | Ja |  |

## Svar

Returnerer: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationPageSearchResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getSearchPages Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetSearchPagesOptions options;
options.pageNumber = boost::optional<int>(1);
options.pageSize = boost::optional<int>(50);
options.searchTerm = boost::optional<utility::string_t>(U("spam"));

api->getSearchPages(U("my-tenant-123"), options)
    .then([](pplx::task<std::shared_ptr<ModerationPageSearchResponse>> task) {
        try {
            auto response = task.get();
            // brug svar
        } catch (const std::exception& e) {
            // håndter fejl
        }
    });
[inline-code-end]