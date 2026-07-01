## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Одговор

Враћа: [`GetQuestionConfigsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Primer getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchQuestionConfigs() {
    const tenantId: string = "tenant-9876";

    // Call without the optional 'skip' parameter
    const configsWithoutSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId);

    // Call with the optional 'skip' parameter
    const skip: number = 10;
    const configsWithSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId, skip);

    console.log(configsWithoutSkip, configsWithSkip);
}
[inline-code-end]