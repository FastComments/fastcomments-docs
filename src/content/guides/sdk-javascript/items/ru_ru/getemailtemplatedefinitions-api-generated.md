## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|-------------|----------|
| tenantId | string | Yes |  |

## Ответ

Возвращает: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEmailTemplates() {
    const tenantId: string = "acme-corp-456";
    const result: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
    const status: APIStatus = result.status;
    const definitions: EmailTemplateDefinition[] = result.definitions ?? [];
    console.log(`Status: ${status.code}, Templates: ${definitions.length}`);
}
[inline-code-end]

---