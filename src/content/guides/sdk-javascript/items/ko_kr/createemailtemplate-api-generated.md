## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| createEmailTemplateBody | CreateEmailTemplateBody | 예 |  |

## 응답

반환: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## 예제

[inline-code-attrs-start title = 'createEmailTemplate 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a9f2b3d";

const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "Comment Notification",
  subject: "New comment on your article: {{postTitle}}",
  htmlBody: "<p>{{commenterName}} left a comment:</p><blockquote>{{commentText}}</blockquote>",
  enabled: true,
  defaultLocale: "en-US",
  metadata: { createdBy: "admin@example.com", purpose: "notify_comment" } // 선택적 추가 데이터
};

const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]

---