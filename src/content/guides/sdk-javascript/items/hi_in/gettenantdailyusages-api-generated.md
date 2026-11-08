## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| yearNumber | number | नहीं |  |
| monthNumber | number | नहीं |  |
| dayNumber | number | नहीं |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsagesResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getTenantDailyUsages उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetTenantDailyUsages() {
  const tenantId: string = "tenant_9876";
  const yearNumber: number = 2024;
  const monthNumber: number = 2;
  const dayNumber: number = 28;
  const skip: number = 10;

  const fullResult: GetTenantDailyUsagesResponse = await getTenantDailyUsages(
    tenantId,
    yearNumber,
    monthNumber,
    dayNumber,
    skip
  );

  const partialResult: GetTenantDailyUsagesResponse = await getTenantDailyUsages(
    tenantId,
    yearNumber
  );
}
[inline-code-end]