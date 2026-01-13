## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createEmailTemplateBody | CreateEmailTemplateBody | כן |  |

## תגובה

מחזיר: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a9f2b3d";

const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "Comment Notification",
  subject: "New comment on your article: {{postTitle}}",
  htmlBody: "<p>{{commenterName}} left a comment:</p><blockquote>{{commentText}}</blockquote>",
  enabled: true,
  defaultLocale: "en-US",
  metadata: { createdBy: "admin@example.com", purpose: "notify_comment" } // נתונים נוספים אופציונליים
};

const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]

---