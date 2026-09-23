將投票附加到評論，或設定已存在投票的完整狀態。

選項會依 ID 匹配：若傳送的選項帶有現有選項的 ID，則保留其投票（並採用新的標籤與位置），未帶 ID 的選項會被新增，列表中省略的現有選項則會被移除，連同其已投的票一起刪除。

在已有投票的投票中不保留任何現有選項 ID 會刪除所有選項，因此需要設定 `replaceVotes=true`。

## Parameters

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| commentPollPutInput | CommentPollPutInput | 是 |  |
| replaceVotes | boolean | 否 |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

[inline-code-attrs-start title = 'putPoll 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---