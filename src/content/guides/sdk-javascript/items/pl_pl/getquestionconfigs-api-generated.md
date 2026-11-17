## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a4cd2f75d6b3b001e4b3c2a';
const skip: number = 10;

const response: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId, skip);

const configs: QuestionConfig[] = response.configs;
const status: string = response.status;

console.log(`Pobrano ${configs.length} konfiguracji pytań`);
configs.forEach((config: QuestionConfig): void => {
    const customOptions: QuestionConfigCustomOptionsInner[] | undefined = config.customOptions;
    const conditions: QuestionCondition[] | undefined = config.conditions;
    const subConditions: SubQuestionCondition[] | undefined = config.subQuestionConditions;
    console.log(`Pytanie ${config.id}: status=${status}, opcje=${customOptions?.length ?? 0}, podpytania=${subConditions?.length ?? 0}`);
});
[inline-code-end]