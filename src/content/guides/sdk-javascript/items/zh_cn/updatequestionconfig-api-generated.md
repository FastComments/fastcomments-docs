## 参数

| 名称 | 类型 | 是否必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'updateQuestionConfig 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-tenant-84f2';
  const id: string = '5d6a8b2f-1c4e-4a7b-9f3d-e2c123456789';
  const customOption: QuestionConfigCustomOptionsInner = { label: 'Helpful', value: 'helpful' };
  const updateQuestionConfigBody: UpdateQuestionConfigBody = {
    enabled: true,
    title: 'Is this information helpful?',
    // 可选参数示例：
    customOptions: [customOption]
  };
  const result: APIEmptyResponse = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
  console.log(result);
})();
[inline-code-end]