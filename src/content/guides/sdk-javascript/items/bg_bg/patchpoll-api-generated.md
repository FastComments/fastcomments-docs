Редактирайте анкета на място, запазвайки резултатите: променете въпроса, променете етикета на опция, затворете или отворете я отново,
или променете кой може да вижда гласувалите. Опциите се адресират по id – за добавяне, премахване или пренареждане, изпратете пълния списък с PUT.

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| commentPollPatch | CommentPollPatch | Да |  |

## Отговор

Връща: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за patchPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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