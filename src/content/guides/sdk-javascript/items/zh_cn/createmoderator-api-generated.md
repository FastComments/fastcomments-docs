## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createModeratorBody | CreateModeratorBody | 是 |  |

## 响应

返回: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModerator200Response.ts)

## 示例

[inline-code-attrs-start title = 'createModerator 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b21';
const createModeratorBody: CreateModeratorBody = {
  moderator: {
    name: 'Alex Rivera',
    email: 'alex.rivera@fastcomments.io',
    role: 'global_moderator',
    enabled: true,
  },
  // 演示可选参数：
  notifyUser: true,
  permissions: ['delete_comment', 'edit_comment', 'ban_user'],
  customConfig: { dashboardTheme: 'dark' } as unknown as CustomConfigParameters
};
const result: CreateModerator200Response = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---