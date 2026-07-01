## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| yearNumber | number | No |  |
| monthNumber | number | No |  |
| dayNumber | number | No |  |
| skip | number | No |  |

## 回應

返回：[`GetTenantDailyUsagesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsagesResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getTenantDailyUsages 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchDailyUsage() {
  const tenantId: string = "tenant-9876";
  const yearNumber: number = 2024;
  const monthNumber: number = 5; // 五月
  const dayNumber: number = 12;
  const skip: number = 0;

  const fullResult: GetTenantDailyUsagesResponse1 = await getTenantDailyUsages(
    tenantId,
    yearNumber,
    monthNumber,
    dayNumber,
    skip
  );

  // 只使用必填參數和一個可選參數
  const partialResult: GetTenantDailyUsagesResponse1 = await getTenantDailyUsages(
    tenantId,
    yearNumber
  );

  console.log(fullResult, partialResult);
}

fetchDailyUsage();
[inline-code-end]