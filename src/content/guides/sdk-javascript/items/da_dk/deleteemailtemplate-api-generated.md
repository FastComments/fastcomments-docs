## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'deleteEmailTemplate Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4c9d1e';
const templateId: string = 'tmpl_welcome_2024-03';
const notifyAdmin: boolean | undefined = true; // eksempel på valgfrit parameter

const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, templateId);
[inline-code-end]

---