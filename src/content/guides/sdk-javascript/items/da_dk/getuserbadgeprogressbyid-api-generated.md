## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`GetUserBadgeProgressByIdResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressByIdResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getUserBadgeProgressById Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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