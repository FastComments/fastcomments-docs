Poll에 투표를 기록하거나 기존 투표를 다른 옵션으로 이동합니다. 투표자는 각 poll당 최대 하나의 투표만 할 수 있으므로, 동일한 투표자에 대해 다시 호출하면 투표가 추가되는 것이 아니라 이동됩니다.

이는 사이트의 poll 설정을 따릅니다: 투표가 로그인한 사용자만 가능하도록 설정된 경우, anonUserId만 있는 투표는 거부되며, 익명 투표는 poll당 IP별로 속도 제한됩니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createPollVoteBody | CreatePollVoteBody | 예 |  |

## 응답

반환: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## 예시

[inline-code-attrs-start title = 'createPollVote 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId는 선택 사항이며 여기서는 생략되었습니다
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]

---