## Параметри

| Име | Тип | Обavezно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`GetPollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoteResponse.ts)

## Пример

[inline-code-attrs-start title = 'getPollVote Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPollVote(): Promise<void> {
  const tenantId: string = "acme-corp-tenant";
  const pollId: string = "poll-2024-05";
  const result: GetPollVoteResponse = await getPollVote(tenantId, pollId);
  const status: APIStatus = result.status;
  const vote: PublicPollVote | undefined = result.vote;
}
[inline-code-end]

---