## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createQuestionConfigBody | CreateQuestionConfigBody | 예 |  |

## 응답

반환: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## 예제

[inline-code-attrs-start title = 'createQuestionConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  required: false, // 선택적: 선택적 필드를 생략한 경우와 포함한 경우를 보여줍니다
  renderingType: QuestionRenderingType.Dropdown,
  options: customOptions,
  whenSave: QuestionWhenSave.Always
};
const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]

---