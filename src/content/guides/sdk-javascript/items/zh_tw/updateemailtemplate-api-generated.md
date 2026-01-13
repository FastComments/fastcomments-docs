## 參數

| 名稱 | 類型 | 是否必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'updateEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const id: string = 'email_tpl_42b7a9';
const updateEmailTemplateBody: UpdateEmailTemplateBody = {
  name: 'Comment Flag Notification',
  subject: 'A comment was flagged on acme.com',
  html: '<p>A comment by {{commenterName}} was flagged. Review at {{moderationUrl}}</p>',
  replyTo: 'noreply@acme.com', // 示範可選欄位
  enabled: true,
  customConfig: { priority: 'high' } // 可選的自訂參數
};
const response: FlagCommentPublic200Response = await updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
[inline-code-end]

---