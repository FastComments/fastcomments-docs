## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'updateQuestionConfig の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-82b3a';
const id: string = 'qst-20260112';
const updateQuestionConfigBody: UpdateQuestionConfigBody = { label: 'Age verification', required: true, renderingType: 'singleChoice', customOptions: [{ value: '18-24', label: '18–24' }] } as UpdateQuestionConfigBody;
const result: FlagCommentPublic200Response = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
[inline-code-end]

---