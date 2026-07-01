Informazioni di massa sugli utenti per un tenant. Dati gli userIds, restituisce le informazioni di visualizzazione da User / SSOUser.  
Utilizzato dal widget dei commenti per arricchire gli utenti appena comparsi tramite un evento di presenza.  
Nessun contesto di pagina: la privacy è applicata uniformemente (i profili privati sono mascherati).

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| ids | String | Sì |  |

## Risposta

Restituisce: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]