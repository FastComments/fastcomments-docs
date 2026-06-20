---
Informazioni degli utenti in blocco per un tenant. Dato userIds, restituisce informazioni di visualizzazione da User / SSOUser.
Utilizzato dal widget dei commenti per arricchire gli utenti che sono appena comparsi tramite un evento di presenza.
Nessun contesto di pagina: la privacy è applicata in modo uniforme (i profili privati sono mascherati).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| ids | string | Sì |  |

## Risposta

Restituisce: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> statusFilter = U("active");
api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try {
        auto res = t.get();
        if (res) {
            auto responseCopy = std::make_shared<PageUsersInfoResponse>(*res);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]

---