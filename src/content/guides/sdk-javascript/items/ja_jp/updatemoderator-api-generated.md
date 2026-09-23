## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateModeratorBody | UpdateModeratorBody | Yes |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'updateModerator の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const moderatorId: string = "mod_12345";

const updateBody: UpdateModeratorBody = {
  isActive: true,
  // role?: string はオプションであり、ここでは省略されています
};

const result: APIEmptyResponse = await updateModerator(tenantId, moderatorId, updateBody);
[inline-code-end]

---