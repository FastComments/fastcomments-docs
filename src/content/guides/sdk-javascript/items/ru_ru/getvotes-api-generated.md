## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |

## Ответ

Возвращает: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f8e91c2';
const urlId: string = 'https://www.sportsdaily.com/news/2026/06/15/championship-game-recap';
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
console.log(votes);
[inline-code-end]

---