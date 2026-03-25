## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerators200Response.ts)

## 예제

[inline-code-attrs-start title = 'getModerators 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-12345-prod';
const moderatorsPage1: GetModerators200Response = await getModerators(tenantId);
const moderatorsPage2: GetModerators200Response = await getModerators(tenantId, 50);
[inline-code-end]

---