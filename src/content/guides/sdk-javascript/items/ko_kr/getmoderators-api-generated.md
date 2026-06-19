---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | number | 아니요 |  |

## 응답

반환: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getModerators 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme-tenant-98765";
  const moderatorsResponse: GetModeratorsResponse = await getModerators(tenantId);
  const skip: number = 25;
  const pagedResponse: GetModeratorsResponse = await getModerators(tenantId, skip);
  console.log(moderatorsResponse, pagedResponse);
}
run();
[inline-code-end]

---