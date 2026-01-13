## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createModeratorBody | CreateModeratorBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModerator200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_us-east_01";
const createModeratorBody: CreateModeratorBody = {
  email: "maria.lopez+mod@fastcompany.com",
  username: "mlopez_mod",
  displayName: "Maria Lopez",
  roles: ["content_moderator"],
  notifyOnReports: true,
  metadata: { region: "us-east", team: "community" }
};
const result: CreateModerator200Response = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---