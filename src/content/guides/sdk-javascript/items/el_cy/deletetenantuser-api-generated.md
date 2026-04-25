## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| deleteComments | string | Όχι |  |
| commentDeleteMode | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme_corp_tenant_9f1a2b";
  const id: string = "user_4d2a1b6c";
  const deleteComments: string = "true"; // αφαίρεσε και τα σχόλια του χρήστη
  const commentDeleteMode: string = "permanent"; // "permanent" ή "soft"
  const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
  console.log(result);
}
run();
[inline-code-end]

---