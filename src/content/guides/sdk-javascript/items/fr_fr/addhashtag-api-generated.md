## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | No |  |
| createHashTagBody | CreateHashTagBody | No |  |

## Réponse

Renvoie: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTag200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple addHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const createHashTagBody: CreateHashTagBody = {
  name: 'feature-request',
  label: 'Feature Request',
  color: '#FF5722',
  enabled: true
};
const response: AddHashTag200Response = await addHashTag(tenantId, createHashTagBody);
const responseWithoutTenant: AddHashTag200Response = await addHashTag(undefined, createHashTagBody);
[inline-code-end]

---