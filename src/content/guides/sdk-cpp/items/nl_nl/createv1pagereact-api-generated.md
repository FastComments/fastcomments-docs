## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| title | string | Nee |  |

## Antwoord

Retourneert: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## Voorbeeld

[inline-code-attrs-start title = 'createV1PageReact Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-abc-789");
boost::optional<utility::string_t> title = boost::optional<utility::string_t>(U("Introducing New Features"));

api->createV1PageReact(tenantId, urlId, title)
    .then([](pplx::task<std::shared_ptr<CreateV1PageReact>> task) {
        try {
            auto result = task.get();
            if (result) {
                auto copy = std::make_shared<CreateV1PageReact>(*result);
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---