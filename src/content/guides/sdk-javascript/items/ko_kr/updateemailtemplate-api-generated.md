## Parameters

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | 예 |  |

## Response

반환: [`UpdateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateEmailTemplateResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateEmailTemplate 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "email_tpl_67890";

  const updateBody: UpdateEmailTemplateBody = {
    subject: "Comment reply notification",
    htmlContent: "<p>Someone replied to your comment.</p>",
    plainTextContent: "Someone replied to your comment.",
    // 선택적 필드 예시
    isActive: true,
  };

  const result: UpdateEmailTemplateResponse = await updateEmailTemplate(
    tenantId,
    templateId,
    updateBody
  );

  console.log(result);
})();
[inline-code-end]