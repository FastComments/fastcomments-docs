## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| sendEmail | string | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8f3b2a7c';
  const moderatorId: string = 'mod_4c12f9b2';
  const responseWithoutEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId);
  const responseWithEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId, 'true');
  console.log(responseWithoutEmail, responseWithEmail);
})();
[inline-code-end]

---