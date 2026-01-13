## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## 示例

[inline-code-attrs-start title = 'getEmailTemplate 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-enterprises-123';
  const id: string = 'welcome-email-template-v2';
  const locale: string | undefined = 'en-US'; // 可选参数示例
  const template: GetEmailTemplate200Response = await getEmailTemplate(tenantId, id);
  console.log(template, locale);
})();
[inline-code-end]

---