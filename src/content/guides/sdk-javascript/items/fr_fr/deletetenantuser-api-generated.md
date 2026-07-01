## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | string | No |  |
| commentDeleteMode | string | No |  |

## Réponse

Renvoie : [`DeleteTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantUserResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple deleteTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoDeleteTenantUser() {
  const tenantId: string = "acme-corp-tenant";
  const userId: string = "user-9876";

  // Supprimer l'utilisateur et tous ses commentaires, en utilisant le mode de suppression définitive
  const resultWithOptions: DeleteTenantUserResponse = await deleteTenantUser(
    tenantId,
    userId,
    "true",
    "hard"
  );

  // Supprimer l'utilisateur sans supprimer les commentaires (comportement par défaut)
  const resultBasic: DeleteTenantUserResponse = await deleteTenantUser(tenantId, userId);
}
[inline-code-end]