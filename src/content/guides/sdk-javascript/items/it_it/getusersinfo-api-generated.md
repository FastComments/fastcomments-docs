Informazioni utente in blocco per un tenant. Dati i userIds, restituisce le informazioni di visualizzazione da User / SSOUser.
Utilizzato dal widget dei commenti per arricchire gli utenti appena apparsi tramite un evento di presenza.
Nessun contesto di pagina: la privacy viene applicata in modo uniforme (i profili privati sono mascherati).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| ids | string | Sì |  |

## Risposta

Restituisce: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // opzionale; se undefined usa la virgola di default
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---