Quem votou em qual opção. Enquetes anônimas nunca respondem, enquetes apenas para administradores respondem somente para um administrador ou moderador.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| urlId | string | Não |  |
| page | number | Não |  |
| search | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoterNamesSuccessResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPollVoters'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

console.log(`Total de eleitores na página: ${voters.length}`);
[inline-code-end]