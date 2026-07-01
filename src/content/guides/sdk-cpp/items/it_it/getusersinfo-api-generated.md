Informazioni utente in blocco per un tenant. Dati gli userIds, restituisce le informazioni da visualizzare da User / SSOUser.  
Utilizzato dal widget dei commenti per arricchire gli utenti appena comparsi tramite un evento di presenza.  
Nessun contesto di pagina: la privacy è applicata uniformemente (i profili privati sono mascherati).

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Risposta

Restituisce: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Esempio

[inline-code-attrs-start title = 'getUsersInfo Esempio'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // process response
    }catch(const std::exception&){
        // handle error
    }
});
[inline-code-end]

---