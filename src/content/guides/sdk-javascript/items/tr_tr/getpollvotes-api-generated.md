The individual votes behind one poll's tallies, oldest first.

Bir anketin sayımlarının arkasındaki bireysel oylar, en eski oylardan başlayarak.

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

Bir anket bir yoruma aittir, bu yüzden oylar her zaman bir anket başına okunur - commentId gereklidir. Bu, her sorgunun koleksiyonun zaten sahip olduğu indekslerde kalmasını sağlar.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

Anketin gizliliğine uyar: anonim bir anketin oyları okunamaz (poll-anonymous), burada ya da kimlik üzerinden.

## Parameters

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| voterId | string | Hayır |  |
| optionId | string | Hayır |  |
| skip | number | Hayır |  |

## Response

## Yanıt

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

Döndürür: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Example

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

---