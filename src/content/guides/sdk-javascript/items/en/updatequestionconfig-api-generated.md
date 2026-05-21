## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateQuestionConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-927';
const id: string = 'questionCfg-4f1d';
const customOption: QuestionConfigCustomOptionsInner = { label: 'Mobile app', value: 'mobile_users' } as QuestionConfigCustomOptionsInner;
const updateQuestionConfigBody: UpdateQuestionConfigBody = {
  title: 'Post-purchase satisfaction',
  enabled: true,
  customOptions: [customOption],
  description: 'Collect quick feedback after checkout'
} as UpdateQuestionConfigBody;
const result: FlagCommentPublic200Response = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
[inline-code-end]
