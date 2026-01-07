[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de mettre à jour un `Tenant` par `id`.

La mise à jour d'un `Tenant` a les restrictions suivantes :

- Les valeurs suivantes ne peuvent pas être mises à jour :
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
  - `managedByTenantId`
- Le `signUpDate` ne peut pas être dans le futur.
- Le `name` ne peut pas dépasser `200 caractères`.
- L'`email` ne peut pas dépasser `300 caractères`.
- L'`email` doit être unique parmi tous les locataires de FastComments.com.
- Lors de la définition de `billingInfoValid` à `true`, `billingInfo` doit être fourni dans la même requête.
- Vous ne pouvez pas mettre à jour le `packageId` associé à votre propre locataire.
- Vous ne pouvez pas mettre à jour le `paymentFrequency` associé à votre propre locataire.

[inline-code-attrs-start title = 'Exemple cURL de PATCH de Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête PATCH de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse PATCH de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
