## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`GetUserBadgeProgressByIdResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressByIdResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getUserBadgeProgressById Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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