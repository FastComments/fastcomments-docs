## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// Προαιρετικές παράμετροι (αν υποστηρίζονται) μπορούν να περαστούν ως δεύτερο όρισμα, π.χ. getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]

---