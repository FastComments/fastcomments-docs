## Παράμετροι

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| questionId | string | Όχι |  |
| questionIds | Array<string> | Όχι |  |
| urlId | string | Όχι |  |
| timeBucket | AggregateTimeBucket | Όχι |  |
| startDate | Date | Όχι |  |
| forceRecalculate | boolean | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα aggregateQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_98765';
const questionIds: Array<string> = ['q-102', 'q-103'];
const urlId: string = 'url_55b3';
const timeBucket: AggregateTimeBucket = { unit: 'day', size: 7 };
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;

const result: AggregateQuestionResults200Response = await aggregateQuestionResults(
  tenantId,
  undefined, // το questionId παραλείπεται, αντί γι' αυτό χρησιμοποιούνται τα questionIds
  questionIds,
  urlId,
  timeBucket,
  startDate,
  forceRecalculate
);
[inline-code-end]

---