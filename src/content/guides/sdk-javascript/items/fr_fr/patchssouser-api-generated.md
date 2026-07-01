## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Oui |  |
| updateComments | boolean | Non |  |

## Réponse

Renvoie : [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchSSOUserAPIResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple patchSSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1f2a3b4-5678-90ab-cdef-1234567890ab";
const userId: string = "user-987654321";
const updateData: UpdateAPISSOUserData = {
  email: "jane.smith@enterprise.com",
  displayName: "Jane Smith",
  role: "editor"
};

const responseWithComments: PatchSSOUserAPIResponse = await patchSSOUser(
  tenantId,
  userId,
  updateData,
  true
);

const responseWithoutComments: PatchSSOUserAPIResponse = await patchSSOUser(
  tenantId,
  userId,
  updateData
);
[inline-code-end]