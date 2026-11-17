Кой гласува за коя опция. Анонимните анкети никога не връщат отговор, а анкетите само за администратори връщат отговор само за администратор или модератор.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| urlId | string | Не |  |
| page | number | Не |  |
| search | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoterNamesSuccessResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getPollVoters'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';

const result: GetPollVoterNamesSuccessResponse = await getPollVoters(
  tenantId,
  'comment_123456',
  'https://example.com/blog/my-post',
  1,
  'jane.doe',
  'SSO_TOKEN_98765'
);

const status: APIStatus = result.status;
const voters: PollVoterEntry[] = result.voters;

console.log(`Общо гласували на страницата: ${voters.length}`);
[inline-code-end]