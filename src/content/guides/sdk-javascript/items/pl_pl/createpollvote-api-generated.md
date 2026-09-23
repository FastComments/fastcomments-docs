Zarejestruj głos w ankiecie lub przenieś istniejący na inną opcję. Głosujący może mieć maksymalnie jeden głos na ankietę, więc wywołanie tego ponownie dla tego samego głosującego przenosi jego głos zamiast dodawać nowy.

To respektuje ustawienia ankiety w serwisie: jeśli głosowanie jest dostępne tylko dla zalogowanych użytkowników, głos z samym anonUserId jest odrzucany, a anonimowe głosy są limitowane według IP na ankietę.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Example

[inline-code-attrs-start title = 'createPollVote Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId is optional and omitted here
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]