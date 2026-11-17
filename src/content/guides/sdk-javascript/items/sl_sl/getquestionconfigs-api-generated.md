## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | number | Ne |  |

## Odziv

Vrne: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a4cd2f75d6b3b001e4b3c2a';
const skip: number = 10;

const response: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId, skip);

const configs: QuestionConfig[] = response.configs;
const status: string = response.status;

console.log(`Pridobljenih ${configs.length} konfiguracij vprašanj`);
configs.forEach((config: QuestionConfig): void => {
    const customOptions: QuestionConfigCustomOptionsInner[] | undefined = config.customOptions;
    const conditions: QuestionCondition[] | undefined = config.conditions;
    const subConditions: SubQuestionCondition[] | undefined = config.subQuestionConditions;
    console.log(`Vprašanje ${config.id}: status=${status}, možnosti=${customOptions?.length ?? 0}, podvprašanja=${subConditions?.length ?? 0}`);
});
[inline-code-end]