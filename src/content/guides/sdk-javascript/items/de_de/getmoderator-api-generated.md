## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Gibt zurück: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getModerator Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_eu_4f8d2b9e";
const maybeModeratorId: string | undefined = "mod_91c3b7a2"; // optionale Quelle (kann undefined sein)
const moderator: GetModerator200Response = await getModerator(tenantId, maybeModeratorId!);
[inline-code-end]

---