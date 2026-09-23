Registre um voto em uma enquete ou mova um voto existente para uma opção diferente. Um eleitor tem no máximo um voto por enquete, portanto chamar isso novamente para o mesmo eleitor move seu voto em vez de adicionar um novo.

Isso segue as configurações de enquete do site: se a votação estiver configurada apenas para usuários autenticados, um voto com apenas um anonUserId é rejeitado, e votos anônimos são limitados por taxa por IP por enquete.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| createPollVoteBody | CreatePollVoteBody | Sim |  |

## Resposta

Retorna: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId é opcional e omitido aqui
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]

---