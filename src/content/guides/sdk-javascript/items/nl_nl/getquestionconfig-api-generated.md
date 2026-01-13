## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Retourneert: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getQuestionConfig Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-47';
const questionId: string = 'q-2026-01-12-01';
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, questionId);
function summarizeConfig(cfg: GetQuestionConfig200Response, includeMetadata?: boolean): QuestionConfig | undefined {
  // includeMetadata is optioneel; implementatie weggelaten voor beknoptheid
  return undefined;
}
const summarizedWithMeta: QuestionConfig | undefined = summarizeConfig(result, true);
const summarizedDefault: QuestionConfig | undefined = summarizeConfig(result);
[inline-code-end]