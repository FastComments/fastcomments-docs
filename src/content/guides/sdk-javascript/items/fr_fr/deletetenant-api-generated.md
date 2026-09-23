## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| sure | string | Non |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple deleteTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_9876";

  const resultWithoutSure: APIEmptyResponse = await deleteTenant(tenantId, userId);
  const resultWithSure: APIEmptyResponse = await deleteTenant(tenantId, userId, "confirm");
}

main();
[inline-code-end]

---