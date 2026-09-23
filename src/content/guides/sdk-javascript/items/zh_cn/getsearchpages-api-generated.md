## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| value | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationPageSearchResponse.ts)

## 示例

[inline-code-attrs-start title = 'getSearchPages 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const searchValue: string = "spam";
const ssoToken: string = "sso-abc123def456";

const result1: ModerationPageSearchResponse = await getSearchPages(tenantId);
const result2: ModerationPageSearchResponse = await getSearchPages(tenantId, searchValue);
const result3: ModerationPageSearchResponse = await getSearchPages(tenantId, searchValue, ssoToken);
[inline-code-end]

---