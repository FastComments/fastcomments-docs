## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| sendEmail | string | Non |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8f3b2a7c';
  const moderatorId: string = 'mod_4c12f9b2';
  const responseWithoutEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId);
  const responseWithEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId, 'true');
  console.log(responseWithoutEmail, responseWithEmail);
})();
[inline-code-end]

---