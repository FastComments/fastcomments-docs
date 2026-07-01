## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Ja |  |
| userId | string | Nej |  |

## Svar

Returnerer: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateSubscriptionAPIResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'updateSubscription Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const subscriptionId: string = "sub_98765";

const updateData: UpdateAPIUserSubscriptionData = {
  planId: "premium_plan",
  status: "active",
  renewalDate: "2024-12-31",
};

const userId: string = "user_abcde";

const response: UpdateSubscriptionAPIResponse = await updateSubscription(
  tenantId,
  subscriptionId,
  updateData,
  userId
);
[inline-code-end]