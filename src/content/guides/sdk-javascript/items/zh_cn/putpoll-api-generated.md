将投票附加到评论，或设置其已有投票的完整状态。

选项通过 id 匹配：带有现有选项 id 的选项会保留其投票（并采用新的标签和位置），未提供 id 的选项会被添加，列表中省略的现有选项会被删除，连同其已投的票一起删除。

在已有投票的投票中不保留任何现有选项 id 会删除所有选项，因此需要将 replaceVotes 设置为 true。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| commentPollPutInput | CommentPollPutInput | 是 |  |
| replaceVotes | boolean | 否 |  |

## 响应

返回：[`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## 示例

[inline-code-attrs-start title = 'putPoll 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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