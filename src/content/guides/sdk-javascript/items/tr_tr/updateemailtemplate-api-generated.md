## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Evet |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateEmailTemplate Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_82b1f";
const id: string = "emailTemplate_9f3b2c";
const updateEmailTemplateBody: UpdateEmailTemplateBody = {
  name: "Comment Notification",
  subject: "New comment on your article",
  html: "<p>Hello,</p><p>You have a new comment on your article. <a href=\"https://example.com\">View</a></p>",
  enabled: true
};
const result: APIEmptyResponse = await updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
[inline-code-end]

---