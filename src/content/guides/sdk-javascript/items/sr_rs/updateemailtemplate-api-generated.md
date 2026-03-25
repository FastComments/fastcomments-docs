## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'updateEmailTemplate Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_76a4b2";
const id: string = "template_9f3c1e";
const updateEmailTemplateBody: UpdateEmailTemplateBody = {
  name: "Comment Flag Notification",
  subject: "A comment was flagged on your-site.com",
  bodyHtml: "<p>Admin,</p><p>User \{{commenterName}} flagged a comment: “\{{commentText}}”</p>",
  isEnabled: true,
  description: "Email sent to moderators when a comment is flagged (optional field included)"
};
const result: FlagCommentPublic200Response = await updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
[inline-code-end]

---