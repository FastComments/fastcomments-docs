## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenantId | string | Так |  |
| createCommentParams | Array<CreateCommentParams> | Так |  |
| isLive | boolean | Ні |  |
| doSpamCheck | boolean | Ні |  |
| sendEmails | boolean | Ні |  |
| populateNotifications | boolean | Ні |  |

## Відповідь

Повертає: `Array<SaveCommentsBulkResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад saveCommentsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const bulkComments: CreateCommentParams[] = [
  {
    content: "Welcome to the new forum thread!",
    authorId: "user_42",
    mentions: [{ userId: "user_84", username: "alice" }],
    hashtags: [{ tag: "intro" }]
  },
  {
    content: "Please review the updated guidelines.",
    authorId: "moderator_1",
    mentions: [],
    hashtags: [{ tag: "guidelines" }, { tag: "update" }]
  }
];

const results: SaveCommentsBulkResponse[] = await saveCommentsBulk(
  tenantId,
  bulkComments,
  true,      // isLive
  false,     // doSpamCheck
  true,      // sendEmails
  undefined  // populateNotifications (за замовчуванням)
);
[inline-code-end]