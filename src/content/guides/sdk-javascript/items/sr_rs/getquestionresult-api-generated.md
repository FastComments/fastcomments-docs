## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## Пример

[inline-code-attrs-start title = 'getQuestionResult Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const post: { title: string; questionId?: string } = { title: 'Product feedback' };
const tenantId: string = 'acme-corp-tenant-01';
const id: string = post.questionId ?? 'q-8f3a7b2c4d9e';
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, id);
[inline-code-end]

---