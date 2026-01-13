## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'updateEmailTemplate 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const id: string = 'email_tpl_42b7a9';
const updateEmailTemplateBody: UpdateEmailTemplateBody = {
  name: 'Comment Flag Notification',
  subject: 'A comment was flagged on acme.com',
  html: '<p>A comment by {{commenterName}} was flagged. Review at {{moderationUrl}}</p>',
  replyTo: 'noreply@acme.com', // 선택적 필드 예시
  enabled: true,
  customConfig: { priority: 'high' } // 선택적 사용자 정의 매개변수
};
const response: FlagCommentPublic200Response = await updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
[inline-code-end]

---