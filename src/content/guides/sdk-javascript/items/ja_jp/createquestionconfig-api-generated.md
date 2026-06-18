---
## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createQuestionConfigBody | CreateQuestionConfigBody | はい |  |

## レスポンス

戻り値: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## 例

[inline-code-attrs-start title = 'createQuestionConfig の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f3b1c2a";

const createQuestionConfigBody: CreateQuestionConfigBody = {
  name: "Product feedback",
  key: "product_quality",
  description: "Short survey question shown after posting a comment",
  required: true,
  renderingType: "singleChoice",
  customOptions: [
    { label: "Excellent", value: "5" },
    { label: "Good", value: "4" },
    { label: "Fair", value: "3" }
  ] as QuestionConfigCustomOptionsInner[],
  notifyModerators: false // オプションのパラメータの例を示す
};

const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]

---