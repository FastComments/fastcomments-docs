투표 집계를 유지한 채 투표를 그 자리에서 편집합니다: 질문을 변경하거나, 옵션의 라벨을 수정하거나, 투표를 마감 또는 재개하거나, 투표자를 볼 수 있는 대상을 변경할 수 있습니다. 옵션은 id로 식별되며, 옵션을 추가, 삭제 또는 재정렬하려면 전체 목록을 PUT으로 전송해야 합니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| commentPollPatch | CommentPollPatch | 예 |  |

## 응답

반환값: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## 예제

[inline-code-attrs-start title = 'patchPoll 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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