## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## 例

[inline-code-attrs-start title = 'getEmailTemplates の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f4d2b7c';
const responseWithoutSkip: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
const skip: number = 20;
const responseWithSkip: GetEmailTemplates200Response = await getEmailTemplates(tenantId, skip);
[inline-code-end]