Hangi seçeneğe kimin oy verdiği. Anonim anketler asla yanıt vermez, yalnızca yöneticilere açık anketler yalnızca bir yönetici veya moderatör için yanıt verir.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| urlId | string | Hayır |  |
| page | number | Hayır |  |
| search | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoterNamesSuccessResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getPollVoters Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

console.log(`Toplam sayfadaki oy veren sayısı: ${voters.length}`);
[inline-code-end]