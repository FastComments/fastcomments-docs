Poszczególne głosy składowe na sumę głosów jednej ankiety, od najstarszych.

Ankieta należy do komentarza, więc głosy są zawsze odczytywane dla jednej ankiety naraz - commentId jest wymagane. Dzięki
temu każde zapytanie korzysta z indeksów, które kolekcja już posiada.

Respektuje prywatność ankiety: głosów w anonimowej ankiecie nie można odczytać (poll-anonymous), ani tutaj, ani po id.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| voterId | string | Nie |  |
| optionId | string | Nie |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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