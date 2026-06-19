## Parametreler

| Name | Type | Required | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Evet |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateQuestionConfig Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-tenant-84f2';
  const id: string = '5d6a8b2f-1c4e-4a7b-9f3d-e2c123456789';
  const customOption: QuestionConfigCustomOptionsInner = { label: 'Helpful', value: 'helpful' };
  const updateQuestionConfigBody: UpdateQuestionConfigBody = {
    enabled: true,
    title: 'Is this information helpful?',
    // isteğe bağlı parametre örneği:
    customOptions: [customOption]
  };
  const result: APIEmptyResponse = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
  console.log(result);
})();
[inline-code-end]