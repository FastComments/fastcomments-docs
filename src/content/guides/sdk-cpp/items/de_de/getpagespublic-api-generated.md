Listet die Seiten für einen Tenant auf. Wird vom FChat-Desktop-Client verwendet, um seine Raumliste zu füllen.
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist.
Seiten, die SSO erfordern, werden gegen den Gruppenzugriff des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| options | const GetPagesPublicOptions& | Ja |  |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // Antwort bei Bedarf verarbeiten
    }catch(const std::exception&){
        // Fehler bei Bedarf behandeln
    }
});
[inline-code-end]