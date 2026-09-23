투표를 취소합니다. 투표가 이루어진 옵션이 그 집계 수를 되돌립니다.

## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`DeletePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePollVoteResponse.ts)

## 예시

[inline-code-attrs-start title = 'deletePollVote 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePollVoteExample(): Promise<void> {
  const tenantId: string = "tenant_42abc";
  const pollId: string = "poll_7f9e2d";

  const result: DeletePollVoteResponse = await deletePollVote(tenantId, pollId);

  console.log(result);
}
[inline-code-end]

---