## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | 是 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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