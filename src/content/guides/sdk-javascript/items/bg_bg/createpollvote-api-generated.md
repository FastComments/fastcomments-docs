Запишете глас в анкета, или преместете съществуващ глас към различна опция. Гласуващият може да има най‑много един глас за анкета, така че повторното извикване за същия гласуващ премества гласа им, вместо да добавя нов.

Това спазва настройките на анкетата в сайта: ако гласуването е зададено само за вписани потребители, глас с само anonUserId се отказва, а анонимните гласове са ограничени по скорост за IP адрес за всяка анкета.

## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Response

Връща: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Example

[inline-code-attrs-start title = 'createPollVote Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---