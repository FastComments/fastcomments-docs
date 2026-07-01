## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|---------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Ja |  |
| updateComments | boolean | Nein |  |

## Antwort

Rückgabe: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchSSOUserAPIResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'patchSSOUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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