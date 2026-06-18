## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## 예제

[inline-code-attrs-start title = 'getCachedNotificationCount 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-23';
const id: string = 'user_987654';
const cachedCount: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, id);

const maybeId: string | undefined = Math.random() > 0.5 ? 'user_123456' : undefined;
if (maybeId) {
  const optionalCachedCount: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, maybeId);
}
[inline-code-end]

---