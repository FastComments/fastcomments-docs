## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`DeleteEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteEmailTemplate Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_abcde";

  const response: DeleteEmailTemplateResponse = await deleteEmailTemplate(tenantId, templateId);

  // Primer pristupa opcionom svojstvu iz odgovora
  const statusCode: number | undefined = response.status?.code;
}();
[inline-code-end]

---