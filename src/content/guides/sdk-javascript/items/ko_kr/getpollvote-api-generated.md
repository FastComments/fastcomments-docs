---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetPollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoteResponse.ts)

## 예시

[inline-code-attrs-start title = 'getPollVote 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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