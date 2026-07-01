## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| userId | string | 否 |  |
| direction | SortDirections | 否 |  |
| repliesToUserId | string | 否 |  |
| page | number | 否 |  |
| includei10n | boolean | 否 |  |
| locale | string | 否 |  |
| isCrawler | boolean | 否 |  |

## 回應

Returns: [`GetCommentsForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getCommentsForUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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