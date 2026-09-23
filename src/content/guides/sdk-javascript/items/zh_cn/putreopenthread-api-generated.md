---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| sso | string | No |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'putReopenThread 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function reopenThreadExample() {
    const tenantId: string = "tenant_12345";
    const urlId: string = "post-9876";
    const sso: string = "user-abc123";

    const responseWithoutSso: APIEmptyResponse = await putReopenThread(tenantId, urlId);
    const responseWithSso: APIEmptyResponse = await putReopenThread(tenantId, urlId, sso);
}

reopenThreadExample();
[inline-code-end]

---