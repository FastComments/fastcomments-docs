## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|-----------|
| urlId | string | Ναι |  |
| tenantId | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`PutCloseThreadResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutCloseThreadResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα putCloseThread'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function closeThreadDemo(): Promise<void> {
  const urlId: string = "article-2023-09-15";
  const tenantId: string = "tenant-42";
  const sso: string = "sso-token-xyz";

  const response: PutCloseThreadResponse = await putCloseThread(urlId, tenantId, sso);
  console.log(response);
}

closeThreadDemo();
[inline-code-end]