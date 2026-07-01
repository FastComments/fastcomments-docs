## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| skip | number | Non |  |

## Réponse

Renvoie : [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // offset de pagination facultatif
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // Exemple sans pagination :
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]

---