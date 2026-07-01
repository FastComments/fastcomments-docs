## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| page | number | No |  |
| count | number | No |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 応答

返り値: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## 例

[inline-code-attrs-start title = 'getApiComments の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // page
    25,                    // count
    "feedback",           // textSearch
    "192.168.1.100",      // byIPFromComment
    "approved",           // filters
    "hasReplies",         // searchFilters
    "dateDesc",           // sorts
    false,                // demo
    "tenant-abc123",      // tenantId
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]