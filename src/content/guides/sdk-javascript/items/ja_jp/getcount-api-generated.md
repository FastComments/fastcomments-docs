## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| textSearch | string | いいえ |  |
| byIPFromComment | string | いいえ |  |
| filter | string | いいえ |  |
| searchFilters | string | いいえ |  |
| demo | boolean | いいえ |  |
| tenantId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`GetCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCountResponse.ts)

## 例

[inline-code-attrs-start title = 'getCount の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const count: GetCountResponse = await getCount({
    textSearch: "order issue",
    byIPFromComment: "198.51.100.23",
    filter: "pending",
    demo: true,
    tenantId: "acme_corp",
    sso: "sso_abcdef123456"
  });
  console.log(count);
}
main();
[inline-code-end]

---