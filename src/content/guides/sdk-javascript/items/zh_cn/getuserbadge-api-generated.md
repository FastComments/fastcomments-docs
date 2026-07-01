## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 响应

返回: [`GetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUserBadge 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
  const tenantId: string = "acme-corp-tenant-001";
  const badgeId: string = "badge-5f9d3a2b";

  const badgeResponse: GetUserBadgeResponse = await getUserBadge(tenantId, badgeId);

  // 安全访问可选字段
  const badgeName: string | undefined = badgeResponse.userBadge?.name;
  console.log(`Badge ID: ${badgeId}, Name: ${badgeName ?? "Unnamed"}`);
}

runExample();
[inline-code-end]