어떤 옵션에 누가 투표했는지 보여줍니다. 익명 투표는 답변하지 않으며, 관리자 전용 투표는 관리자 또는 중재자에게만 답변합니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| urlId | string | 아니요 |  |
| page | number | 아니요 |  |
| search | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환값: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoterNamesSuccessResponse.ts)

## 예제

[inline-code-attrs-start title = 'getPollVoters 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';

const result: GetPollVoterNamesSuccessResponse = await getPollVoters(
  tenantId,
  'comment_123456',
  'https://example.com/blog/my-post',
  1,
  'jane.doe',
  'SSO_TOKEN_98765'
);

const status: APIStatus = result.status;
const voters: PollVoterEntry[] = result.voters;

console.log(`Total voters on page: ${voters.length}`);
[inline-code-end]