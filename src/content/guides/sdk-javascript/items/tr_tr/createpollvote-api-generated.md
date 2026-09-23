Poll üzerinde bir oy kaydedin veya mevcut oyu farklı bir seçeneğe taşıyın. Bir seçmen, bir anket başına en fazla bir oy alabilir, bu yüzden aynı seçmen için bu fonksiyon tekrar çağrıldığında oy eklemek yerine mevcut oyları taşır.

Bu, sitenin anket ayarlarına uyar: eğer oy verme yalnızca oturum açmış kullanıcılarla sınırlıysa, yalnızca bir `anonUserId` içeren oy reddedilir ve anonim oylar IP başına anket başına sınırlanır.

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Yanıt

Döndürür: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Örnek

[inline-code-attrs-start title = 'createPollVote Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId isteğe bağlıdır ve burada atlanmıştır
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]