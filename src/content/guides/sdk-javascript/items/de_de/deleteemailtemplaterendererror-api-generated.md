## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| errorId | string | Ja |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b4c2a';
const templateEnvironment: string | undefined = 'production'; // optionaler Umgebungsselektor
const id: string = `emailTemplates/${templateEnvironment ?? 'staging'}/welcome_v2`;
const errorId: string = 'err_5a9d2f1c';
const result: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
console.log(result);
[inline-code-end]

---