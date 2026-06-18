## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createModeratorBody | CreateModeratorBody | はい |  |

## レスポンス

戻り値: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModerator200Response.ts)

## 例

[inline-code-attrs-start title = 'createModerator の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b21';
const createModeratorBody: CreateModeratorBody = {
  moderator: {
    name: 'Alex Rivera',
    email: 'alex.rivera@fastcomments.io',
    role: 'global_moderator',
    enabled: true,
  },
  // オプションのパラメータの例:
  notifyUser: true,
  permissions: ['delete_comment', 'edit_comment', 'ban_user'],
  customConfig: { dashboardTheme: 'dark' } as unknown as CustomConfigParameters
};
const result: CreateModerator200Response = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---