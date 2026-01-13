## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример употребе getQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-47';
const questionId: string = 'q-2026-01-12-01';
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, questionId);
function summarizeConfig(cfg: GetQuestionConfig200Response, includeMetadata?: boolean): QuestionConfig | undefined {
  // includeMetadata је опционално; имплементација је изостављена ради краткоће
  return undefined;
}
const summarizedWithMeta: QuestionConfig | undefined = summarizeConfig(result, true);
const summarizedDefault: QuestionConfig | undefined = summarizeConfig(result);
[inline-code-end]

---