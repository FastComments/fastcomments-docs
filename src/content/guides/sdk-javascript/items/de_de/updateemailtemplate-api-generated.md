## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Ja |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'updateEmailTemplate Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const id: string = 'email_tpl_42b7a9';
const updateEmailTemplateBody: UpdateEmailTemplateBody = {
  name: 'Comment Flag Notification',
  subject: 'A comment was flagged on acme.com',
  html: '<p>A comment by {{commenterName}} was flagged. Review at {{moderationUrl}}</p>',
  replyTo: 'noreply@acme.com', // optionales Feld zur Demonstration
  enabled: true,
  customConfig: { priority: 'high' } // optionale benutzerdefinierte Parameter
};
const response: FlagCommentPublic200Response = await updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
[inline-code-end]

---