## Παράμετροι

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Απόκριση

Returns: [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // προαιρετικό offset σελιδοποίησης
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // Παράδειγμα χωρίς σελιδοποίηση:
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]

---