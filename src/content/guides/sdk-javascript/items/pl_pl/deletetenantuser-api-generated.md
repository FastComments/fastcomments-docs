## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | string | No |  |
| commentDeleteMode | string | No |  |

## Odpowiedź

Zwraca: [`DeleteTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantUserResponse.ts)

## Przykład

[inline-code-attrs-start title = 'deleteTenantUser Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoDeleteTenantUser() {
  const tenantId: string = "acme-corp-tenant";
  const userId: string = "user-9876";

  // Usuń użytkownika i wszystkie jego komentarze, używając trybu twardego usuwania
  const resultWithOptions: DeleteTenantUserResponse = await deleteTenantUser(
    tenantId,
    userId,
    "true",
    "hard"
  );

  // Usuń użytkownika bez usuwania komentarzy (domyślne zachowanie)
  const resultBasic: DeleteTenantUserResponse = await deleteTenantUser(tenantId, userId);
}
[inline-code-end]