---
## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| deleteComments | string | Nee |  |
| commentDeleteMode | string | Nee |  |

## Respons

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteTenantUser Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme_corp_tenant_9f1a2b";
  const id: string = "user_4d2a1b6c";
  const deleteComments: string = "true"; // verwijder ook de reacties van de gebruiker
  const commentDeleteMode: string = "permanent"; // "permanent" of "soft"
  const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
  console.log(result);
}
run();
[inline-code-end]

---