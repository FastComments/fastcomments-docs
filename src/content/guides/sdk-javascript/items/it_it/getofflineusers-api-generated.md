---
Commentatori precedenti sulla pagina che NON sono attualmente online. Ordinati per displayName.  
Usa questo dopo aver esaurito /users/online per rendere una sezione "Members".  
Paginazione a cursore su commenterName: il server percorre il parziale {tenantId, urlId, commenterName} indice da afterName in avanti tramite $gt, senza costo $skip.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Risposta

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]

---