## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const idSuffix: string | undefined = undefined;
const tenantId: string = "acme-enterprises";
const id: string = idSuffix ?? "user_98765";
const response: GetUser200Response = await getUser({ tenantId, id });
[inline-code-end]

---