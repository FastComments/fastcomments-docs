## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createModeratorBody | CreateModeratorBody | 是 |  |

## 回應

回傳: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## 範例

[inline-code-attrs-start title = 'createModerator 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_83f4b7a2';
const createModeratorBody: CreateModeratorBody = {
  email: 'renee.alvarez@acme-corp.com',
  fullName: 'Renee Alvarez',
  roles: ['content_moderator'],
  notify: true // 示範可選參數
};
const result: CreateModeratorResponse = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---