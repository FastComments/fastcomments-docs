Ποιος ψήφισε για ποια επιλογή. Τα ανώνυμα ψηφοφορίες δεν απαντούν ποτέ, οι ψηφοφορίες μόνο για διαχειριστές απαντούν μόνο για διαχειριστή ή συντονιστή.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| urlId | string | Όχι |  |
| page | number | Όχι |  |
| search | string | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoterNamesSuccessResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPollVoters'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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