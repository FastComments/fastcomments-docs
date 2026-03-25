## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| questionId | string | No |  |
| questionIds | Array<string> | No |  |
| urlId | string | No |  |
| timeBucket | AggregateTimeBucket | No |  |
| startDate | Date | No |  |
| forceRecalculate | boolean | No |  |

## Respuesta

Devuelve: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de aggregateQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_98765';
const questionIds: Array<string> = ['q-102', 'q-103'];
const urlId: string = 'url_55b3';
const timeBucket: AggregateTimeBucket = { unit: 'day', size: 7 };
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;

const result: AggregateQuestionResults200Response = await aggregateQuestionResults(
  tenantId,
  undefined, // questionId omitido, usando questionIds en su lugar
  questionIds,
  urlId,
  timeBucket,
  startDate,
  forceRecalculate
);
[inline-code-end]

---