## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`GetModeratorResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-001";
  const id: string = "mod-12345";

  const result: GetModeratorResponse1 = await getModerator(tenantId, id);

  const moderatorName: string | undefined = result.moderator?.name;
  console.log(moderatorName);
})();
[inline-code-end]