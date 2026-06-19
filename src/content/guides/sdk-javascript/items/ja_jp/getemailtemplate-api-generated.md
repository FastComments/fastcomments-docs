## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse.ts)

## 例

[inline-code-attrs-start title = 'getEmailTemplate の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-1";
const templateId: string = "welcome-onboard-v2";
const result: GetEmailTemplateResponse = await getEmailTemplate(tenantId, templateId);
const status: APIStatus | undefined = result.status;
const template: CustomEmailTemplate | undefined = result.template;
const subject: string | undefined = template?.subject
[inline-code-end]

---