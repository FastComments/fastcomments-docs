---
## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteQuestionResult пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-publishing-01';
const id: string = 'qres-7a3d9f45-2b6e-4c9a-8f1b-0d3f2c9a1e6b';
const result: APIEmptyResponse = await deleteQuestionResult(tenantId, id);
[inline-code-end]

---