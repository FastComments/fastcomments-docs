Запишите глас у анкети, или преместите постојећи глас на другу опцију. Гласач може имати највише један глас по анкети, тако да поновним позивом за истог гласача премешта се њихов глас уместо да се дода нови.

Ово поштује подешавања анкете на сајту: ако је гласање омогућено само пријављеним корисницима, глас са само anonUserId се одбија, а анонимни гласови су ограничени по брзини по IP по анкети.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createPollVoteBody | CreatePollVoteBody | Да |  |

## Одговор

Враћа: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Пример

[inline-code-attrs-start title = 'createPollVote Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId је опционо и изостављено овде
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]