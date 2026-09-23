Уреди анкету на месту, задржавајући њене резултате: промени питање, преименуј опцију, затвори или поново отвори, или промени ко може да види гласаче. Опције се адресирају по id – за додавање, уклањање или промену редоследа, **PUT** пуну листу.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| commentPollPatch | CommentPollPatch | Да |  |

## Одговор

Враћа: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Пример

[inline-code-attrs-start title = 'patchPoll Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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