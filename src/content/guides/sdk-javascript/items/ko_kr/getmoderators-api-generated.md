## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## 응답

반환: [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## 예시

[inline-code-attrs-start title = 'getModerators 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // 선택적 페이지네이션 오프셋
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // 페이지네이션 없이 예시:
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]