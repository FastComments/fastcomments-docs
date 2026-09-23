## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| userId | string | Tak |  |

## Odpowiedź

Zwraca: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserBadgeProgressByUserId'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const userId: string = "user-42";

const badgeProgress: APIGetUserBadgeProgressResponse = await getUserBadgeProgressByUserId(tenantId, userId);

if (badgeProgress.status?.code !== undefined) {
  const statusCode: number = badgeProgress.status.code;
}

if (badgeProgress.progress?.length) {
  const firstProgress: UserBadgeProgress = badgeProgress.progress[0];
}
[inline-code-end]