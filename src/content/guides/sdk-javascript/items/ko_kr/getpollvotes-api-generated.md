The individual votes behind one poll's tallies, oldest first.

하나의 설문 조사 집계 뒤에 있는 개별 투표들, 오래된 순서대로.

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That keeps every query on the indexes the collection already has.

설문 조사는 댓글에 속하므로, 투표는 항상 한 번에 하나의 설문 조사씩 읽습니다 - commentId가 필요합니다. 이렇게 하면 컬렉션이 이미 가지고 있는 인덱스에서 모든 쿼리를 수행하게 됩니다.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

설문 조사 프라이버시를 준수합니다: 익명 설문 조사(poll-anonymous)의 투표는 여기서든 ID로든 읽을 수 없습니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| voterId | string | 아니오 |  |
| optionId | string | 아니오 |  |
| skip | number | 아니오 |  |

## 응답

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## 예시

[inline-code-attrs-start title = 'getPollVotes 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]

---