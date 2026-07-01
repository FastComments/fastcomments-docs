List sider for en lejer. Bruges af FChat‑desktopklienten til at udfylde sin rumliste.  
Kræver `enableFChat` at være sand på den løste brugerdefinerede konfiguration for hver side.  
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| options | const GetPagesPublicOptions& | Ja |  |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Example

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // behandle svar om nødvendigt
    }catch(const std::exception&){
        // håndter fejl om nødvendigt
    }
});
[inline-code-end]