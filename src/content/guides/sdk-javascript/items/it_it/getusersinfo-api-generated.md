Informazioni utente in blocco per un tenant. Dati gli userIds, restituisce le informazioni di visualizzazione da User / SSOUser.
Utilizzato dal widget dei commenti per arricchire gli utenti appena comparsi tramite un evento di presenza.
Nessun contesto di pagina: la privacy viene applicata in modo uniforme (i profili privati sono mascherati).

## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| ids | string | Sì |  |

## Risposta

Restituisce: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo richiede solo tenantId e ids; i parametri opzionali non sono applicabili qui.
[inline-code-end]

---