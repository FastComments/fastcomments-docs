## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | 是 |  |
| locale | string | 否 |  |

## 回應

返回：[`RenderEmailTemplateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplateResponse1.ts)

## 範例

[inline-code-attrs-start title = 'renderEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-01";
  const templateBody: RenderEmailTemplateBody = {
    templateId: "welcome-email",
    placeholders: {
      userName: "John Doe",
      signupDate: "2024-04-01"
    }
  };
  const locale: string = "en-US";
  const result: RenderEmailTemplateResponse1 = await renderEmailTemplate(tenantId, templateBody, locale);
  console.log(result);
})();
[inline-code-end]

---