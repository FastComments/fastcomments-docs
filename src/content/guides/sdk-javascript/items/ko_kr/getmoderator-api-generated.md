## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## 예제

[inline-code-attrs-start title = 'getModerator 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media-58';
const id: string = 'mod-82f3b9c1';
const moderatorResponse: GetModerator200Response = await getModerator(tenantId, id);
[inline-code-end]

---