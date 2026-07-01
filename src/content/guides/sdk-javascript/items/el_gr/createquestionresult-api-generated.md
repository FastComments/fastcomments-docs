## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## Απάντηση

Επιστρέφει: [`CreateQuestionResultResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResultResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";

const metaItem: MetaItem = {
  key: "campaign",
  value: "spring-launch"
};

const questionResultBody: CreateQuestionResultBody = {
  questionId: "question-42",
  answer: "Positive",
  metadata: [metaItem]
  // προαιρετικά πεδία όπως σημειώσεις παραλείπονται
};

const result: CreateQuestionResultResponse1 = await createQuestionResult(tenantId, questionResultBody);
[inline-code-end]