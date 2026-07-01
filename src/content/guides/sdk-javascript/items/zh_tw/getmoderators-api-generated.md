## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## 回應

返回：[`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getModerators 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // 可選的分頁偏移
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // 沒有分頁的範例：
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]