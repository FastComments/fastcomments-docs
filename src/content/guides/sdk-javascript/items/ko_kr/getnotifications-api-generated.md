## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | boolean | No |  |
| type | string | No |  |
| skip | number | No |  |

## 응답

반환: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse.ts)

## 예시

[inline-code-attrs-start title = 'getNotifications 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchNotifications(): Promise<void> {
  const tenantId: string = "tenant_42";
  const userId: string = "user_1001";
  const urlId: string = "url_2023";
  const viewed: boolean = true;
  const skip: number = 0;

  const notifications: GetNotificationsResponse = await getNotifications(
    tenantId,
    userId,
    urlId,
    undefined,
    viewed,
    undefined,
    skip
  );

  console.log(notifications);
}
[inline-code-end]

---