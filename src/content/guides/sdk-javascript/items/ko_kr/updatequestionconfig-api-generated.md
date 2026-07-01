## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Yes |  |

## 응답

반환: [`UpdateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionConfigResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateQuestionConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const questionId: string = "qstn-2023-04";

const updateBody: UpdateQuestionConfigBody = {
  // 선택적 필드 예시
  customOptions: [
    {
      id: "opt-001",
      label: "Extra Details",
      required: true,
    },
  ],
  renderingType: "markdown",
};

const response: UpdateQuestionConfigResponse = await updateQuestionConfig(
  tenantId,
  questionId,
  updateBody
);
[inline-code-end]