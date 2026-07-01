Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare l'elenco delle sue stanze.  
Richiede che `enableFChat` sia impostato su true nella configurazione personalizzata risolta per ogni pagina.  
Le pagine che richiedono SSO sono filtrate in base all'accesso al gruppo dell'utente richiedente.

## Parameters

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| options | const GetPagesPublicOptions& | Yes |  |

## Response

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Example

[inline-code-attrs-start title = 'Esempio getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // elabora la risposta se necessario
    }catch(const std::exception&){
        // gestisci l'errore se necessario
    }
});
[inline-code-end]