## 參數

| 名稱 | 類型 | 必要 | 描述 |
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

## 回應

回傳：[`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getApiComments 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // 頁面
    25,                    // 數量
    "feedback",           // 文字搜尋
    "192.168.1.100",      // 依IP取得評論
    "approved",           // 篩選條件
    "hasReplies",         // 搜尋篩選條件
    "dateDesc",           // 排序
    false,                // 示範
    "tenant-abc123",      // 租戶ID
    "sso-token-xyz"       // 單點登入
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]

---