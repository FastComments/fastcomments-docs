## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | לא |  |
| createHashTagBody | CreateHashTagBody | לא |  |

## Response

מחזיר: [`AddHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagResponse.ts)

## Example

[inline-code-attrs-start title = 'דוגמת addHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9876";

const newHashTag: CreateHashTagBody = {
  tag: "typescript",
  description: "Discussions about TypeScript"
};

const responseWithTenant: AddHashTagResponse = await addHashTag(tenantId, newHashTag);

const responseWithoutTenant: AddHashTagResponse = await addHashTag(undefined, newHashTag);
[inline-code-end]