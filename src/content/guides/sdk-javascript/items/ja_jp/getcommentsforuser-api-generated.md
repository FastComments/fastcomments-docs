## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| userId | string | いいえ |  |
| direction | SortDirections | いいえ |  |
| repliesToUserId | string | いいえ |  |
| page | number | いいえ |  |
| includei10n | boolean | いいえ |  |
| locale | string | いいえ |  |
| isCrawler | boolean | いいえ |  |

## レスポンス

返却: [`GetCommentsForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse1.ts)

## 例

[inline-code-attrs-start title = 'getCommentsForUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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