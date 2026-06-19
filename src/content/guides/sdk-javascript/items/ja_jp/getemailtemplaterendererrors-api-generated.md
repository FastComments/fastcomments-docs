## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrorsResponse.ts)

## 例

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org_3f6a9b2c-45d8-4e1a";
const templateId: string = "tmpl_9b1c7d6e-22f4-4a3b";
const errorsWithoutSkip: GetEmailTemplateRenderErrorsResponse = await getEmailTemplateRenderErrors(tenantId, templateId);
const errorsWithSkip: GetEmailTemplateRenderErrorsResponse = await getEmailTemplateRenderErrors(tenantId, templateId, 50);
[inline-code-end]

---