Qui a voté pour quelle option. Les sondages anonymes ne répondent jamais, les sondages réservés aux administrateurs ne répondent que pour un administrateur ou un modérateur.

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| urlId | string | Non |  |
| page | number | Non |  |
| search | string | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoterNamesSuccessResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getPollVoters'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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