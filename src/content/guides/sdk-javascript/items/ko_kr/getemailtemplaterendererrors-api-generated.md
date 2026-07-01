## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| skip | number | No |  |

## 응답

반환: [`GetEmailTemplateRenderErrorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrorsResponse1.ts)

## 예시

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c2f5a8d9-4b3e-4f6a-9e1b-2d5c6f7a8b9c";
const templateId: string = "welcome-email-template";
const skip: number = 20;

const result: GetEmailTemplateRenderErrorsResponse1 = await getEmailTemplateRenderErrors(
  tenantId,
  templateId,
  skip
);
[inline-code-end]