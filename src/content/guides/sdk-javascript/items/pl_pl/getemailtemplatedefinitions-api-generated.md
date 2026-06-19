## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |

## Odpowiedź

Zwraca: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// Opcjonalne parametry (jeśli obsługiwane) można przekazać jako drugi argument, np. getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]

---