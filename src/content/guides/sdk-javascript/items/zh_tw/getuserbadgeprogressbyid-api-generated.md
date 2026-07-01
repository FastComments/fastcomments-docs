---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetUserBadgeProgressByIdResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressByIdResponse.ts)

## 範例

[inline-code-attrs-start title = 'getUserBadgeProgressById 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo(): Promise<void> {
    const tenantId: string = "acme-corp";
    const userId: string = "user-42";
    const result: GetUserBadgeProgressByIdResponse = await getUserBadgeProgressById(tenantId, userId);
    const progress: UserBadgeProgress | undefined = result.progress;
    const earnedAt: Date | undefined = progress?.earnedAt;
    console.log(`Badge earned at: ${earnedAt?.toISOString() ?? "not earned yet"}`);
}
demo();
[inline-code-end]

---