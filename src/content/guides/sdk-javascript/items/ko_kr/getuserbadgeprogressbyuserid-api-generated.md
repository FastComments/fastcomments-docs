## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 예 |  |

## 응답

반환: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-inc-tenant-01';
const userId: string = 'user_73c9b2';
const progress: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, userId);

async function maybeFetchProgress(tenant: string, user?: string): Promise<GetUserBadgeProgressById200Response | null> {
  if (!user) return null;
  return await getUserBadgeProgressByUserId(tenant, user);
}

const optionalResult: GetUserBadgeProgressById200Response | null = await maybeFetchProgress(tenantId, userId);
[inline-code-end]

---