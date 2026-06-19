## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'deleteEmailTemplate 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const templateId: string = 'tmpl_3fa85f64-5717-4562-b3fc-2c963f66afa6';
const optionalStatus: APIStatus | undefined = undefined;
const result: APIEmptyResponse = await deleteEmailTemplate(tenantId, templateId);
[inline-code-end]

---