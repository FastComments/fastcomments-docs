## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| deleteComments | string | Nein |  |
| commentDeleteMode | string | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteTenantUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme_corp_tenant_9f1a2b";
  const id: string = "user_4d2a1b6c";
  const deleteComments: string = "true"; // entfernt auch die Kommentare des Benutzers
  const commentDeleteMode: string = "permanent"; // "permanent" oder "soft"
  const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
  console.log(result);
}
run();
[inline-code-end]

---