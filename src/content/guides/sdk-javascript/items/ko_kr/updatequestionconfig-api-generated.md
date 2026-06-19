## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | 예 |  |

## 반환

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateQuestionConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-tenant-84f2';
  const id: string = '5d6a8b2f-1c4e-4a7b-9f3d-e2c123456789';
  const customOption: QuestionConfigCustomOptionsInner = { label: 'Helpful', value: 'helpful' };
  const updateQuestionConfigBody: UpdateQuestionConfigBody = {
    enabled: true,
    title: 'Is this information helpful?',
    // 선택적 매개변수 예시:
    customOptions: [customOption]
  };
  const result: APIEmptyResponse = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
  console.log(result);
})();
[inline-code-end]