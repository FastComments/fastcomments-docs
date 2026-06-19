## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Sì |  |
| updateComments | string | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di replaceTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-92";
const id: string = "user_7f9b2a";
const replaceTenantUserBody: ReplaceTenantUserBody = {
  email: "maria.garcia@acme-corp.com",
  displayName: "María García",
  role: "moderator",
  externalId: "ext-5271"
};
const updateComments: string = "true";
const result: APIEmptyResponse = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]