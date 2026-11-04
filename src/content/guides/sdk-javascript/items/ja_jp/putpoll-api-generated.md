コメントに投票(ポール)を添付したり、既に添付されている投票の状態全体を設定します。

選択肢はidによって照合されます。既存の選択肢と同じidを持つ選択肢を送信すると、その投票数は保持され(新しいラベルと位置が適用され)、idなしで送信された選択肢は追加され、リストに含まれていない既存の選択肢は、それに対して投じられた投票とともに削除されます。

投票が存在する投票で既存の選択肢のidを1つも保持しない場合、それらはすべて削除されるため、replaceVotes=trueが必要です。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| commentPollPutInput | CommentPollPutInput | はい |  |
| replaceVotes | boolean | いいえ |  |

## レスポンス

戻り値: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## 例

[inline-code-attrs-start title = 'putPoll の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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