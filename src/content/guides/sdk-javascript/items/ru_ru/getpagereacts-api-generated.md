Пользовательские счётчики реакций для страницы, опционально с информацией о пользователях, которые отреагировали.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| includeUserInfo | PageReactIncludeUserInfo | Нет |  |

## Ответ

Возвращает: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageReactsAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPageReacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'my-blog-post-42';
const includeUserInfo: PageReactIncludeUserInfo = true;

const response: GetPageReactsAPIResponse = await getPageReacts(
  tenantId,
  urlId,
  includeUserInfo
);

const reacts = response.pageReacts;
const reactCount: number = reacts.like.count;
const users: PageCustomReactUser[] = reacts.like.userDetails ?? [];
[inline-code-end]