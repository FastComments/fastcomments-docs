## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createUserBadgeParams | CreateUserBadgeParams | Ναι |  |

## Απάντηση

Επιστρέφει: [`CreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadgeResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'createUserBadge Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "company-42";

const badgeParams: CreateUserBadgeParams = {
  name: "Community Champion",
  iconUrl: "https://assets.example.com/badges/champion.png",
  // η περιγραφή είναι προαιρετική και παραλείπεται εδώ
};

const result: CreateUserBadgeResponse = await createUserBadge(tenantId, badgeParams);
[inline-code-end]