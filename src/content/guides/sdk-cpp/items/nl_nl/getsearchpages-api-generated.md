## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| options | const GetSearchPagesOptions& | Ja |  |

## Reactie

Retourneert: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationPageSearchResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getSearchPages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetSearchPagesOptions options;
options.pageNumber = boost::optional<int>(1);
options.pageSize = boost::optional<int>(50);
options.searchTerm = boost::optional<utility::string_t>(U("spam"));

api->getSearchPages(U("my-tenant-123"), options)
    .then([](pplx::task<std::shared_ptr<ModerationPageSearchResponse>> task) {
        try {
            auto response = task.get();
            // gebruik response
        } catch (const std::exception& e) {
            // verwerk fout
        }
    });
[inline-code-end]