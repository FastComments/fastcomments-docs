## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updatableCommentParams | UpdatableCommentParams | Yes |  |
| contextUserId | string | No |  |
| doSpamCheck | boolean | No |  |
| isLive | boolean No |  |

## Одговор

Враћа: [`UpdateCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'updateComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const updatableCommentParams: UpdatableCommentParams = {
  // пример поља; стварни облик зависи од дефиниције API‑ја
  // нпр., body: "Edited comment content",
};

const contextUserId: string = "user_abcde";
const doSpamCheck: boolean = true;
const isLive: boolean = false;

const result: UpdateCommentResponse = await updateComment(
  tenantId,
  commentId,
  updatableCommentParams,
  contextUserId,
  doSpamCheck,
  isLive
);
[inline-code-end]