## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createModeratorBody | CreateModeratorBody | 是 |  |

## 响应

返回: [`CreateModeratorResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse1.ts)

## 示例

[inline-code-attrs-start title = 'createModerator 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const moderatorPayload: CreateModeratorBody = {
    name: "Alice Johnson",
    email: "alice.johnson@example.com"
    // 可选字段如 description 已省略
  };
  const result: CreateModeratorResponse1 = await createModerator(tenantId, moderatorPayload);
  console.log(result);
}

runExample();
[inline-code-end]