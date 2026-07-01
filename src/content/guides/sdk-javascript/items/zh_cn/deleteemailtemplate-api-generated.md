---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回：[`DeleteEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteEmailTemplate 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_abcde";

  const response: DeleteEmailTemplateResponse = await deleteEmailTemplate(tenantId, templateId);

  // 示例：访问响应中的可选属性
  const statusCode: number | undefined = response.status?.code;
}();
[inline-code-end]

---