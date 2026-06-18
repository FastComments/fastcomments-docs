## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createEmailTemplateBody | CreateEmailTemplateBody | 是 |  |

## 回應

回傳：[`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## 範例

[inline-code-attrs-start title = 'createEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4f2b1c9e";
const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "New Comment Notification",
  subject: "Someone replied to your discussion",
  fromName: "Community Team",
  fromAddress: "no-reply@community.example.com",
  htmlBody: "<p>\{{comment.author}} replied: \{{comment.text}}</p>",
  plaintextBody: "\{{comment.author}} replied: \{{comment.text}}",
  previewText: "A new reply on a discussion you follow",
  isDefault: false // 可選旗標，示範可選參數的用法
};
const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]

---