## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| yearNumber | number | いいえ |  |
| monthNumber | number | いいえ |  |
| dayNumber | number | いいえ |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsagesResponse.ts)

## 例

[inline-code-attrs-start title = 'getTenantDailyUsages の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "b4f3a9c2-8d1e-4f3b-9c6e-2a7f4d5c1e0b";
  const yearNumber: number = 2026;
  const monthNumber: number = 6;
  const dayNumber: number = 19;
  const skip: number = 0;
  const fullResponse: GetTenantDailyUsagesResponse = await getTenantDailyUsages(tenantId, yearNumber, monthNumber, dayNumber, skip);
  const basicResponse: GetTenantDailyUsagesResponse = await getTenantDailyUsages(tenantId);
  console.log(fullResponse, basicResponse);
}
run();
[inline-code-end]

---