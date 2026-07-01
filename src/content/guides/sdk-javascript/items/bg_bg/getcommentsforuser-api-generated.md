## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| userId | string | Не |  |
| direction | SortDirections | Не |  |
| repliesToUserId | string | Не |  |
| page | number | Не |  |
| includei10n | boolean | Не |  |
| locale | string | Не |  |
| isCrawler | boolean | Не |  |

## Отговор

Връща: [`GetCommentsForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse1.ts)

## Пример

[inline-code-attrs-start title = 'getCommentsForUser Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
    const userId: string = "user-12345";
    const direction: SortDirections = "desc";
    const page: number = 1;
    const includei10n: boolean = true;
    const locale: string = "en-US";
    const isCrawler: boolean = false;

    const response: GetCommentsForUserResponse1 = await getCommentsForUser(
        userId,
        direction,
        undefined,
        page,
        includei10n,
        locale,
        isCrawler
    );

    console.log(response);
}
[inline-code-end]

---