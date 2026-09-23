在原位編輯投票，保留其票數：變更問題、重新標記選項、關閉或重新開啟投票，  
或變更誰可以看到投票者。選項以 id 為依據——若要新增、刪除或重新排序，請使用 PUT 完整列表。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| commentPollPatch | CommentPollPatch | 是 |  |

## 回應

返回：[`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## 範例

[inline-code-attrs-start title = 'patchPoll 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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