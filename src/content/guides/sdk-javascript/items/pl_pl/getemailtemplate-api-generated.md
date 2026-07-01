## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Tak |  |

## Odpowiedź

Zwraca: [`GetEmailTemplateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse1.ts)

## Przykład

[inline-code-attrs-start title = 'getEmailTemplate Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTemplate(): Promise<void> {
    const tenantId: string = "tenant-12345";
    const templateId: string = "order-confirmation";
    const response: GetEmailTemplateResponse1 = await getEmailTemplate(tenantId, templateId);
    const emailTemplate: CustomEmailTemplate | undefined = response.customEmailTemplate;
    const configParams: CustomConfigParameters | undefined = response.customConfigParameters;
}
[inline-code-end]

---