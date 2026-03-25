## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## Svar

Returnerer: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'createQuestionResult Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-72b1f4";
const meta: MetaItem[] = [{ key: "platform", value: "web" }];
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "question-83472",
  commenterId: "user-5521",
  answers: [{ subQuestionId: "sq-1", value: "Yes" }],
  meta, // valgfri metadata
  note: "Follow-up requested" // valgfri parameter demonstreret
};
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---