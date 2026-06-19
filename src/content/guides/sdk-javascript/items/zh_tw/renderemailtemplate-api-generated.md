## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | 是 |  |
| locale | string | 否 |  |

## 回應

回傳：[`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplateResponse.ts)

## 範例

[inline-code-attrs-start title = 'renderEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2c44';
const renderEmailTemplateBody: RenderEmailTemplateBody = {
  templateId: 'welcome_v2',
  recipient: { name: 'Lucas Moreno', email: 'lucas@startup.io' },
  variables: { siteName: 'TechDaily', activationLink: 'https://techdaily.io/activate/abc123' }
};
const locale: string = 'en-US';
const result: RenderEmailTemplateResponse = await renderEmailTemplate(tenantId, renderEmailTemplateBody, locale);
[inline-code-end]

---