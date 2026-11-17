## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
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
const tenantId: string = 'demo-rowy';

const commentParams: CreateCommentParams = {
  url: 'https://blog.example.com/announcing-v2-release',
  pageTitle: 'Announcing v2 Release',
  email: 'sarah.johnson@gmail.com',
  author: 'Sarah Johnson',
  comment: 'This update looks amazing! Just tried it on our staging site.',
  zmqUid: '177244808120441',
  userMentions: [{
    email: 'dev.lead@company.com',
    author: 'Alex Lead'
  }] as unknown as CommentUserMentionInfo[]
};

const createCommentParams: Array<CreateCommentParams> = [commentParams];

const responses: Array<SaveCommentsBulkResponse> = await saveCommentsBulk(
  tenantId,
  createCommentParams,
  true,   // isLive
  true,   // doSpamCheck
  false,  // sendEmails
  true    // populateNotifications
);

const firstResponse: SaveCommentsBulkResponse = responses[0];
[inline-code-end]