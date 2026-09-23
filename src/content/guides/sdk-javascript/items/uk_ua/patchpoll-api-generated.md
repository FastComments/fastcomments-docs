Редагуйте опитування на місці, зберігаючи підрахунки: змініть питання, перейменуйте варіант, закрийте або відкрийте його,
або змініть, хто може бачити голосувальників. Варіанти ідентифікуються за id — щоб додати, видалити або змінити їх порядок, PUT повний список.

## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| commentPollPatch | CommentPollPatch | Так |  |

## Відповідь

Повертає: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Приклад

[inline-code-attrs-start title = 'patchPoll Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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