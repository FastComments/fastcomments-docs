Lijst pagina’s voor een tenant. Wordt gebruikt door de FChat‑desktopclient om zijn kamerlijst te vullen. Vereist `enableFChat` om true te zijn in de opgeloste aangepaste configuratie voor elke pagina. Pagina’s die SSO vereisen, worden gefilterd op basis van de groepsrechten van de aanvragende gebruiker.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| options | const GetPagesPublicOptions& | Ja |  |

## Respons

Retourneert: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getPagesPublic Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // process response if needed
    }catch(const std::exception&){
        // handle error if needed
    }
});
[inline-code-end]