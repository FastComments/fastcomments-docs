---
## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |

## Одговор

Враћа: [`GetPollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollResponse.ts)

## Пример

[inline-code-attrs-start title = 'getPoll пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c9f1e2b4-8a3d-4f6a-9d2e-7b5c1a2f3e4d";
const commentId: string = "a1b2c3d4e5f6g7h8i9j0";

const pollResponse: GetPollResponse = await getPoll(tenantId, commentId);

const poll: CommentPoll | undefined = pollResponse.poll;
const privacy: PollPrivacy | undefined = poll?.privacy;
const options: CommentPollOption[] | undefined = poll?.options;
[inline-code-end]

---