## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## תגובה

מחזיר: [`GetQuestionConfigsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse1.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchQuestionConfigs() {
    const tenantId: string = "tenant-9876";

    // קריאה ללא הפרמטר האופציונלי 'skip'
    const configsWithoutSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId);

    // קריאה עם הפרמטר האופציונלי 'skip'
    const skip: number = 10;
    const configsWithSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId, skip);

    console.log(configsWithoutSkip, configsWithSkip);
}
[inline-code-end]