## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| skip | number | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme-tenant-98765";
  const moderatorsResponse: GetModeratorsResponse = await getModerators(tenantId);
  const skip: number = 25;
  const pagedResponse: GetModeratorsResponse = await getModerators(tenantId, skip);
  console.log(moderatorsResponse, pagedResponse);
}
run();
[inline-code-end]

---