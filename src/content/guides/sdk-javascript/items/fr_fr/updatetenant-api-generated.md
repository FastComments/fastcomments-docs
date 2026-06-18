## Paramètres

| Name | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateTenantBody | UpdateTenantBody | Oui |  |

## Réponse

Renvoie : [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f12';
const id: string = 'flag_192b';
const updateTenantBody: UpdateTenantBody = {
  name: 'Acme Media',
  billingInfo: { plan: 'enterprise', seats: 25, nextBillingDate: '2026-07-01' },
  apiDomainConfiguration: { primaryDomain: 'comments.acme.com', additionalDomains: ['acme.com'] },
  importedSites: [{ siteUrl: 'https://blog.acme.com', archived: false }], // optionnel
  commentSettings: { htmlRenderingMode: 'sanitized', deletionMode: 'soft' } // optionnel
} as UpdateTenantBody;
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]

---