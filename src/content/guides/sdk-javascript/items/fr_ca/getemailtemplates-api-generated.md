## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| skip | number | Non |  |

## Réponse

Renvoie : [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getEmailTemplates'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f4d2b7c';
const responseWithoutSkip: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
const skip: number = 20;
const responseWithSkip: GetEmailTemplates200Response = await getEmailTemplates(tenantId, skip);
[inline-code-end]

---