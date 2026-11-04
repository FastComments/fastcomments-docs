Bir anketin toplamlarının ardındaki tek tek oylar, eskiden yeniye doğru.

Bir anket bir yoruma aittir, bu nedenle oylar her zaman tek bir anket üzerinden okunur - commentId zorunludur. Bu, her sorgunun koleksiyonun zaten sahip olduğu indeksler üzerinde kalmasını sağlar.

Anketin gizlilik ayarına uyar: anonim bir anketin oyları okunamaz (poll-anonymous), ne burada ne de id ile.

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Yanıt

Döndürür: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getPollVotes Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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