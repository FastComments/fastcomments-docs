## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateModeratorBody | UpdateModeratorBody | Ja |  |

## Antwort

Rückgabe: [`UpdateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateModeratorResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'updateModerator Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUpdateModerator(): Promise<void> {
    const tenantId: string = "tenant_42abc";
    const moderatorId: string = "moderator_8f9e";
    const updateBody: UpdateModeratorBody = {
        isActive: true,
        role: "admin",
        // optionales Feld
        notes: "Promoted to senior moderator"
    };
    const result: UpdateModeratorResponse = await updateModerator(tenantId, moderatorId, updateBody);
    console.log(result);
}

demoUpdateModerator();
[inline-code-end]