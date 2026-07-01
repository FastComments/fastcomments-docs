## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|---------------|
| tenantId | string | Ja |  |

## Respons

Retourneert: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPagesAPIResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getPages Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
api->getPages(tenantId)
    .then([](pplx::task<std::shared_ptr<GetPagesAPIResponse>> task) {
        try {
            auto response = task.get();
            boost::optional<utility::string_t> nextCursor = response->nextCursor;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]