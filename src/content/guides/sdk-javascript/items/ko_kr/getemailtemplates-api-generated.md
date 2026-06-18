---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## 예제

[inline-code-attrs-start title = 'getEmailTemplates 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme-marketing-tenant-001";
  const templatesDefault: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
  const templatesPaged: GetEmailTemplates200Response = await getEmailTemplates(tenantId, 25);
  console.log(templatesDefault, templatesPaged);
}
run();
[inline-code-end]

---