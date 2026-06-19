## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createEmailTemplateBody | CreateEmailTemplateBody | 是 |  |

## 回傳

回傳: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplateResponse.ts)

## 範例

[inline-code-attrs-start title = 'createEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_7f9b2a";

const customTemplate: CustomEmailTemplate = {
  id: "custtmpl_01",
  name: "MinimalTransactional",
  html: "<div style=\"font-family:Arial,Helvetica,sans-serif\">\{{body}}</div>"
};

const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "User Welcome - Web",
  subject: "Welcome to Acme — Get Started",
  html: "<p>Hi \{{firstName}}, welcome to Acme!</p>",
  previewText: "Start exploring your new Acme account",
  enabled: true,
  replyTo: "support@acme.com",
  customTemplate // optional parameter demonstrated
};

const result: CreateEmailTemplateResponse = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]

---