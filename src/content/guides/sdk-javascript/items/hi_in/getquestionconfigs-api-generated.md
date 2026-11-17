## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getQuestionConfigs उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a4cd2f75d6b3b001e4b3c2a';
const skip: number = 10;

const response: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId, skip);

const configs: QuestionConfig[] = response.configs;
const status: string = response.status;

console.log(`Fetched ${configs.length} question configs`);
configs.forEach((config: QuestionConfig): void => {
    const customOptions: QuestionConfigCustomOptionsInner[] | undefined = config.customOptions;
    const conditions: QuestionCondition[] | undefined = config.conditions;
    const subConditions: SubQuestionCondition[] | undefined = config.subQuestionConditions;
    console.log(`Question ${config.id}: status=${status}, options=${customOptions?.length ?? 0}, subQuestions=${subConditions?.length ?? 0}`);
});
[inline-code-end]