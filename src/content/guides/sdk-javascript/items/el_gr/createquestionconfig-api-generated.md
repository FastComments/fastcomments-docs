## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_live_7f8b3c2a";
const customOptions: QuestionConfigCustomOptionsInner[] = [
  { value: "under18", label: "Under 18" },
  { value: "18-24", label: "18-24" },
  { value: "25-34", label: "25-34", defaultSelected: true }
];
const createQuestionConfigBody: CreateQuestionConfigBody = {
  key: "age_range",
  label: "What is your age range?",
  required: false, // προαιρετικό: επίδειξη παράλειψης έναντι συμπερίληψης προαιρετικών πεδίων
  renderingType: QuestionRenderingType.Dropdown,
  options: customOptions,
  whenSave: QuestionWhenSave.Always
};
const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]

---