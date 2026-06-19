## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';
const packageId: string = 'pkg_prod_delete_2026-06-19';
const onComplete: ((status?: APIStatus) => void) | undefined = undefined;
const response: APIEmptyResponse = await deleteTenantPackage(tenantId, packageId);
onComplete?.();
[inline-code-end]

---