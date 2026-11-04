Informazioni utente in blocco per un tenant. Dati userIds, restituisce le informazioni di visualizzazione da User / SSOUser.
Utilizzato dal widget dei commenti per arricchire gli utenti appena apparsi tramite un evento di presenza.
Nessun contesto di pagina: la privacy è applicata in modo uniforme (i profili privati sono mascherati).

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| ids | string | Sì |  |

## Risposta

Restituisce: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]