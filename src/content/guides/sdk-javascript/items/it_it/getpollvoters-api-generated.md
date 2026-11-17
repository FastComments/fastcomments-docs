Chi ha votato per quale opzione. I sondaggi anonimi non rispondono mai; i sondaggi riservati agli amministratori rispondono solo per un amministratore o un moderatore.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| urlId | string | No |  |
| page | number | No |  |
| search | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoterNamesSuccessResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getPollVoters'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

console.log(`Total voters on page: ${voters.length}`);
[inline-code-end]