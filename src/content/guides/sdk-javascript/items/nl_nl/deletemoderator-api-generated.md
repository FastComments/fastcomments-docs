## Parameters

| Naam | Type | Verplicht | Omschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| sendEmail | string | Nee |  |

## Reactie

Retourneert: [`DeleteModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteModeratorResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteModerator Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteModeratorExample() {
  const tenantId: string = "tenant_2023";
  const moderatorId: string = "mod_001";
  const notificationEmail: string = "admin@mycompany.com";

  const resultWithEmail: DeleteModeratorResponse = await deleteModerator(tenantId, moderatorId, notificationEmail);
  const resultWithoutEmail: DeleteModeratorResponse = await deleteModerator(tenantId, moderatorId);
}
[inline-code-end]