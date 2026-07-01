## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetSearchPagesOptions& | Yes |  |

## Odpowiedź

Zwraca: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationPageSearchResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getSearchPages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetSearchPagesOptions options;
options.pageNumber = boost::optional<int>(1);
options.pageSize = boost::optional<int>(50);
options.searchTerm = boost::optional<utility::string_t>(U("spam"));

api->getSearchPages(U("my-tenant-123"), options)
    .then([](pplx::task<std::shared_ptr<ModerationPageSearchResponse>> task) {
        try {
            auto response = task.get();
            // użyj odpowiedzi
        } catch (const std::exception& e) {
            // obsłuż błąd
        }
    });
[inline-code-end]