## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| skip | number | Non |  |

## Réponse

Renvoie: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getEmailTemplates'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme-marketing-tenant-001";
  const templatesDefault: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
  const templatesPaged: GetEmailTemplates200Response = await getEmailTemplates(tenantId, 25);
  console.log(templatesDefault, templatesPaged);
}
run();
[inline-code-end]