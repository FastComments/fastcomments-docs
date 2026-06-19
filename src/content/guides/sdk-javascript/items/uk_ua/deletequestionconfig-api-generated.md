## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'deleteQuestionConfig Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f2b3c';
const id: string = 'qcfg_9a8b7c';
const metadataNote: string | undefined = undefined; // необов'язкові метадані (не вимагаються функцією)
const result: APIEmptyResponse = await deleteQuestionConfig(tenantId, id);
metadataNote;
[inline-code-end]

---