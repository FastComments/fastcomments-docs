## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

返却: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultResponse.ts)

## 例

[inline-code-attrs-start title = 'getQuestionResult の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-inc-tenant-7';
const id: string = 'b7f9c3a2-4d1e-4a2f-9c1b-0d5e8f6a9b3c';
const result: GetQuestionResultResponse = await getQuestionResult(tenantId, id);
const status: APIStatus | undefined = result.status;
const questionResult: QuestionResult | undefined = result.questionResult;
const metaItems: MetaItem[] | undefined = result.meta?.items;
[inline-code-end]

---