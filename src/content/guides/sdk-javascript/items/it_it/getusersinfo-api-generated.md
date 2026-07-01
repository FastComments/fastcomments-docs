Bulk user info per un tenant. Dati gli userIds, restituisce le informazioni visualizzabili da User / SSOUser.  
Utilizzato dal widget dei commenti per arricchire gli utenti appena comparsi tramite un evento di presenza.  
Nessun contesto di pagina: la privacy è applicata uniformemente (i profili privati sono mascherati).

## Parameters

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Returns: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// I campi opzionali nella risposta possono essere indefiniti
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]