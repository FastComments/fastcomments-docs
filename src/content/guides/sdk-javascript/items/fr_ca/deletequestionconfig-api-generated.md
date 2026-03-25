## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Retourne : [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-01";
const idOptional: string | undefined = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
const id: string = idOptional ?? "11111111-1111-1111-1111-111111111111";
const response: FlagCommentPublic200Response = await deleteQuestionConfig(tenantId, id);
[inline-code-end]

---