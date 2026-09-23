Attach a poll to a comment, or set the full state of the poll it already has.

Options are matched by id: an option sent with the id of an existing option keeps its votes (and takes the
new label and position), an option sent without an id is added, and existing options left out of the list
are removed along with the votes cast on them.

Keeping no existing option ids on a poll that has votes deletes all of them, so that needs replaceVotes=true.

## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| commentPollPutInput | CommentPollPutInput | 예 |  |
| replaceVotes | boolean | 아니오 |  |

## Response

반환: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

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