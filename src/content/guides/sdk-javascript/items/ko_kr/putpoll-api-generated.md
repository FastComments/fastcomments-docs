댓글에 투표를 추가하거나, 이미 연결된 투표의 전체 상태를 설정합니다.

옵션은 id로 매칭됩니다: 기존 옵션의 id와 함께 전송된 옵션은 기존 투표 수를 유지하며(새 라벨과 위치를 적용), id 없이 전송된 옵션은 추가되고, 목록에서 누락된 기존 옵션은 해당 옵션에 대한 투표와 함께 제거됩니다.

투표가 있는 투표에서 기존 옵션 id를 하나도 유지하지 않으면 모든 투표가 삭제되므로, 이 경우 replaceVotes=true가 필요합니다.

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## 응답

반환값: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## 예제

[inline-code-attrs-start title = 'putPoll 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const commentId: string = "9876543210";

const optionA: CommentPollOptionInput = { text: "Dark mode" };
const optionB: CommentPollOptionInput = { text: "Light mode" };

const pollInput: CommentPollPutInput = {
  question: "Which UI theme do you prefer?",
  options: [optionA, optionB],
};

const replaceVotes: boolean = true;

const result: SavePollResponse = await putPoll(tenantId, commentId, pollInput, replaceVotes);
[inline-code-end]