---
Supprimez un sondage de son commentaire, ainsi que tous les votes qui y ont été castés. Le commentaire lui‑même reste inchangé.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple deletePoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePoll(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6d";
  const commentId: string = "comment_a1b2c3d4";
  const result: APIEmptyResponse = await deletePoll(tenantId, commentId);
  console.log(result);
}
[inline-code-end]

---