## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Yes |  |

## 回應

回傳: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## 範例

[inline-code-attrs-start title = 'createEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_9f4a2b';
  const createEmailTemplateBody: CreateEmailTemplateBody = {
    name: 'Weekly Digest',
    subject: 'Your weekly discussion highlights',
    html: '<!doctype html><body><h1>Hello \{{user.name}}</h1><p>Top comments this week...</p></body>',
    fromAddress: 'no-reply@fastcomments-example.com',
    replyTo: 'moderation@fastcomments-example.com',
    isDefault: false
  };
  const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
  console.log(result);
})();
[inline-code-end]