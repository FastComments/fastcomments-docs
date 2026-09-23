投票をその場で編集し、集計を保持します：質問を変更したり、オプションのラベルを変更したり、閉じたり再開したり、投票者が誰かを見ることができるかを変更したりします。オプションは ID で指定されます - 追加、削除、または順序変更を行う場合は、完全なリストを PUT してください。

## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPatch | CommentPollPatch | Yes |  |

## レスポンス

返却: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## 例

[inline-code-attrs-start title = 'patchPoll の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_9876543210";

  const pollPatch: CommentPollPatch = {
    question: "Which new feature would you like to see?",
    options: [
      { id: "opt-1", label: "Dark Mode" },
      { id: "opt-2", label: "Multi-language Support" }
    ]
  };

  const response: SavePollResponse = await patchPoll(tenantId, commentId, pollPatch);
  console.log(response);
})();
[inline-code-end]

---