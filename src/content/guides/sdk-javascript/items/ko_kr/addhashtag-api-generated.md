## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createHashTagBody | CreateHashTagBody | 아니오 |  |

## 응답

반환: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateHashTagResponse.ts)

## 예시

[inline-code-attrs-start title = 'addHashTag 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c3f5e8b2-9d4a-4f6a-8b2c-1a2b3c4d5e6f";

const tagPayload: CreateHashTagBody = {
  tag: "typescript",
  description: "Discussions about TypeScript"
};

const responseWithBody: CreateHashTagResponse = await addHashTag(tenantId, tagPayload);
const responseWithoutBody: CreateHashTagResponse = await addHashTag(tenantId);
[inline-code-end]

---