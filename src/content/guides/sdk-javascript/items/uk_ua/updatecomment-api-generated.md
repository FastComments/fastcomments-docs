## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| updatableCommentParams | UpdatableCommentParams | Так |  |
| contextUserId | string | Ні |  |
| doSpamCheck | boolean | Ні |  |
| isLive | boolean | Ні |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp_01";
const id: string = "comment_20260325_4592";
const updatableCommentParams: UpdatableCommentParams = {
  body: "Updated the response to include a link to the RFC and fixed a typo in the second paragraph.",
  editedByUserId: "user_8721",
  isVisible: true
};
const contextUserId: string = "user_8721";
const doSpamCheck: boolean = true;
const isLive: boolean = true;
const result: FlagCommentPublic200Response = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]