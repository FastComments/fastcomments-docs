## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Απάντηση

Επιστρέφει: [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // προαιρετικό όριο σελίδωσης
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // Παράδειγμα χωρίς σελίδωση:
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]