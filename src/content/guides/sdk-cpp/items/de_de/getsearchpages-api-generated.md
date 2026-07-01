## Parameter

| Name   | Type                     | Required | Description |
|--------|--------------------------|----------|-------------|
| tenantId | string                  | Ja       |  |
| options  | const GetSearchPagesOptions& | Ja       |  |

## Antwort

Rückgabe: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationPageSearchResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getSearchPages Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetSearchPagesOptions options;
options.pageNumber = boost::optional<int>(1);
options.pageSize = boost::optional<int>(50);
options.searchTerm = boost::optional<utility::string_t>(U("spam"));

api->getSearchPages(U("my-tenant-123"), options)
    .then([](pplx::task<std::shared_ptr<ModerationPageSearchResponse>> task) {
        try {
            auto response = task.get();
            // Antwort verwenden
        } catch (const std::exception& e) {
            // Fehler behandeln
        }
    });
[inline-code-end]

---