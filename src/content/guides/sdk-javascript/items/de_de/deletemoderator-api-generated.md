## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| sendEmail | string | Nein |  |

## Antwort

Rückgabe: [`DeleteModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteModeratorResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteModerator Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteModeratorExample() {
  const tenantId: string = "tenant_2023";
  const moderatorId: string = "mod_001";
  const notificationEmail: string = "admin@mycompany.com";

  const resultWithEmail: DeleteModeratorResponse = await deleteModerator(tenantId, moderatorId, notificationEmail);
  const resultWithoutEmail: DeleteModeratorResponse = await deleteModerator(tenantId, moderatorId);
}
[inline-code-end]

---