## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| questionId | string | Nej |  |
| questionIds | Array<string> | Nej |  |
| urlId | string | Nej |  |
| startDate | Date | Nej |  |
| forceRecalculate | boolean | Nej |  |
| minValue | number | Nej |  |
| maxValue | number | Nej |  |
| limit | number | Nej |  |

## Respons

Returnerer: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_12345';
const questionId: string = 'q_98765';
const questionIds: string[] = ['q_98765', 'q_12345'];
const urlId: string = 'url_abc123';
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const minValue: number = 0;
const maxValue: number = 5;
const limit: number = 50;
const result: CombineCommentsWithQuestionResults200Response = await combineCommentsWithQuestionResults(
  tenantId,
  questionId,
  questionIds,
  urlId,
  startDate,
  forceRecalculate,
  minValue,
  maxValue,
  limit
);
[inline-code-end]

---