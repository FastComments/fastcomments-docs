## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateModeratorBody | UpdateModeratorBody | Oui |  |

## Réponse

Retourne : [`UpdateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateModeratorResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUpdateModerator(): Promise<void> {
    const tenantId: string = "tenant_42abc";
    const moderatorId: string = "moderator_8f9e";
    const updateBody: UpdateModeratorBody = {
        isActive: true,
        role: "admin",
        // champ facultatif
        notes: "Promoted to senior moderator"
    };
    const result: UpdateModeratorResponse = await updateModerator(tenantId, moderatorId, updateBody);
    console.log(result);
}

demoUpdateModerator();
[inline-code-end]