## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwoord

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteEmailTemplate Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4c9d1e';
const templateId: string = 'tmpl_welcome_2024-03';
const notifyAdmin: boolean | undefined = true; // voorbeeld van optionele parameter

const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, templateId);
[inline-code-end]

---