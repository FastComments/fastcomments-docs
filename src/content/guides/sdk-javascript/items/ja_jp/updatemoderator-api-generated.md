## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateModeratorBody | UpdateModeratorBody | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'updateModerator の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-42";
const id: string = "moderator_517";
const updateModeratorBody: UpdateModeratorBody = {
  displayName: "Sofia Martinez",
  email: "sofia.martinez@acme.com",
  permissions: ["approve_comments", "flag_spam", "suspend_users"],
  active: true,
  avatarUrl: "https://cdn.acme.com/avatars/sofia.jpg" // オプションのフィールドを示しています
};
const result: FlagCommentPublic200Response = await updateModerator(tenantId, id, updateModeratorBody);
[inline-code-end]

---