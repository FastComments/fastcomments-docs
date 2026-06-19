## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tag | string | 예 |  |
| tenantId | string | 아니요 |  |
| updateHashTagBody | UpdateHashTagBody | 아니요 |  |

## 응답

반환: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## 예제

[inline-code-attrs-start title = 'patchHashTag 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "release-notes";
const tenantId: string = "tenant_8421";
const updateHashTagBody: UpdateHashTagBody = {
  name: "Release Notes",
  description: "Thread for discussing feature releases and changelogs",
  isActive: true
};
const result: UpdateHashTagResponse = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---