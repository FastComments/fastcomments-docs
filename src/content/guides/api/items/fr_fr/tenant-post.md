[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Cette route fournit la capacité d'ajouter un seul `Tenant`.

La création d'un `Tenant` a les restrictions suivantes :

- Un `name` est requis.
- `domainConfiguration` est requis.
- Les valeurs suivantes ne peuvent pas être fournies lors de la création d'un `Tenant` :
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
- Le `signUpDate` ne peut pas être dans le futur.
- Le `name` ne peut pas dépasser `200 caractères`.
- L'`email` ne peut pas dépasser `300 caractères`.
- L'`email` doit être unique parmi tous les locataires de FastComments.com.
- Vous ne pouvez pas créer de locataires si le locataire parent n'a pas de `TenantPackage` valide défini.
  - Si votre locataire a été créé via FastComments.com, cela ne devrait pas être un problème.
- Vous ne pouvez pas créer plus de locataires que défini sous `maxWhiteLabeledTenants` dans votre forfait.
- Vous devez spécifier le paramètre de requête `tenantId` qui est l'id de votre `locataire parent` avec le white labeling activé.

Nous pouvons créer un `Tenant` avec seulement quelques paramètres :

[inline-code-attrs-start title = 'Exemple cURL de Création de Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Création de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Création de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Included on failure. **/
    reason?: string
    tenant?: Tenant; // We return the complete created tenant on success.
}
[inline-code-end]
