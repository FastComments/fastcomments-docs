## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| userId | string | はい |  |

## レスポンス

返却値: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressResponse.ts)

## 例

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '9f3a2c7e-1b5d-4f8a-9a2e-123456789abc';
const userId: string = 'd4e8f0a1-6b3c-4d2e-8f1b-9876543210fe';
const result: APIGetUserBadgeProgressResponse = await getUserBadgeProgressByUserId(tenantId, userId);
const status: APIStatus | undefined = (result as unknown as { status?: APIStatus }).status;
const badgeProgress: UserBadgeProgress[] | undefined = (result as unknown as { badgeProgress?: UserBadgeProgress[] }).badgeProgress;
[inline-code-end]

---