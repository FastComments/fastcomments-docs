## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Da |  |

## Response

Vrne: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer createQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "acme-corp-123";

  const customOption: QuestionConfigCustomOptionsInner = {
    key: "maxLength",
    value: 500,
  };

  const createQuestionConfigBody: CreateQuestionConfigBody = {
    name: "User Feedback",
    description: "Collect user feedback after purchase",
    enabled: true,
    customOptions: [customOption],
    // neobvezno polje
    tags: ["feedback", "post-purchase"],
  };

  const response: CreateQuestionConfigResponse = await createQuestionConfig(
    tenantId,
    createQuestionConfigBody
  );

  console.log(response);
}
[inline-code-end]

---