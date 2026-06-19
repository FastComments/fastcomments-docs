## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| fromName | string | Oui |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de sendInvite'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'bright-media-12';
const id: string = 'user-8f4d2';
const fromName: string = 'Sofia Park';
const optionalNote: string | undefined = undefined;
const result: APIEmptyResponse = await sendInvite(tenantId, id, fromName);
[inline-code-end]

---