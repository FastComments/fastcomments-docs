---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createEmailTemplateBody | CreateEmailTemplateBody | 예 |  |

## 응답

반환: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## 예제

[inline-code-attrs-start title = 'createEmailTemplate 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---