## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Yes |  |
| locale | string | No |  |

## 응답

Returns: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplateResponse.ts)

## 예시

[inline-code-attrs-start title = 'renderEmailTemplate 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";

const templateBody: RenderEmailTemplateBody = {
  templateId: "welcome-email",
  variables: {
    userName: "Jane Doe",
    signupDate: "2024-04-01"
  }
};

const locale: string = "en-US";

(async () => {
  const rendered: RenderEmailTemplateResponse = await renderEmailTemplate(
    tenantId,
    templateBody,
    locale
  );
  console.log(rendered);
})();
[inline-code-end]