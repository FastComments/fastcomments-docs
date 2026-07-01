## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Evet |  |
| locale | string | Hayır |  |

## Yanıt

Döndürür: [`RenderEmailTemplateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplateResponse1.ts)

## Örnek

[inline-code-attrs-start title = 'renderEmailTemplate Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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