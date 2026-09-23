Појединачни гласови иза збирних резултата једне анкете, од најстаријих према новим.

Анкета припада коментару, тако да се гласови увек читају по једној анкети – потребан је commentId. То одржава сваки упит на индексима које колекција већ има.

Поштује приватност анкете: гласови анонимне анкете не могу да се читају (poll-anonymous), ни овде ни по ID-у.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Одговор

Враћа: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]