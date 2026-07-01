## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`DeleteEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateResponse.ts)

## 예시

[inline-code-attrs-start title = 'deleteEmailTemplate 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_abcde";

  const response: DeleteEmailTemplateResponse = await deleteEmailTemplate(tenantId, templateId);

  // 응답에서 선택적 속성에 접근하는 예시
  const statusCode: number | undefined = response.status?.code;
}();
[inline-code-end]