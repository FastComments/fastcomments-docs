## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | number | Нет |  |
| limit | number | Нет |  |
| skip | number | Нет |  |
| asTree | boolean | Нет |  |
| skipChildren | number | Нет |  |
| limitChildren | number | Нет |  |
| maxTreeDepth | number | Нет |  |
| urlId | string | Нет |  |
| userId | string | Нет |  |
| anonUserId | string | Нет |  |
| contextUserId | string | Нет |  |
| hashTag | string | Нет |  |
| parentId | string | Нет |  |
| direction | SortDirections | Нет |  |
| fromDate | number | Нет |  |
| toDate | number | Нет |  |

## Ответ

Возвращает: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_789";
const page: number = 1;
const limit: number = 25;
const asTree: boolean = true;
const maxTreeDepth: number = 3;
const urlId: string = "articles/2026/fastcomments-intro";
const userId: string = "user_12345";
const direction: SortDirections = "desc";
const fromDate: number = 1672531200000;
const toDate: number = Date.now();

const result: APIGetCommentsResponse = await getComments(
  tenantId,
  page,
  limit,
  0,
  asTree,
  0,
  5,
  maxTreeDepth,
  urlId,
  userId,
  undefined,
  undefined,
  "#release",
  undefined,
  direction,
  fromDate,
  toDate
);
[inline-code-end]

---