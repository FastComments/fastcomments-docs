## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createQuestionConfigBody | CreateQuestionConfigBody | はい |  |

## レスポンス

戻り値: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse.ts)

## 例

[inline-code-attrs-start title = 'createQuestionConfig の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org-7b3d1e9f";
const customOption: QuestionConfigCustomOptionsInner = {
  key: "priority",
  label: "Priority",
  values: ["low", "medium", "high"],
  defaultValue: "medium"
};
const createQuestionConfigBody: CreateQuestionConfigBody = {
  name: "Customer Support Questions",
  description: "Configuration for support-related question flows",
  enabled: true,
  moderation: { required: true, level: "manual" },
  questionLimitPerUser: 5,
  customOptions: [customOption],
  allowAnonymous: false
};
const response: CreateQuestionConfigResponse = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]

---