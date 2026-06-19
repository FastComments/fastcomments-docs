## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Da |  |
| updateComments | string | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer replaceTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---