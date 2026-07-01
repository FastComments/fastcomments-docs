## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | No |  |
| createHashTagBody | CreateHashTagBody | No |  |

## 응답

반환: [`AddHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagResponse.ts)

## 예시

[inline-code-attrs-start title = 'addHashTag 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9876";

const newHashTag: CreateHashTagBody = {
  tag: "typescript",
  description: "Discussions about TypeScript"
};

const responseWithTenant: AddHashTagResponse = await addHashTag(tenantId, newHashTag);

const responseWithoutTenant: AddHashTagResponse = await addHashTag(undefined, newHashTag);
[inline-code-end]