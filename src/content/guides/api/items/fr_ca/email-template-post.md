[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de créer des modèles d'email.

Notes :

- Vous ne pouvez pas avoir plusieurs modèles avec le même `emailTemplateId` avec le même domaine.
- Mais vous pouvez avoir un modèle générique (`domain` = `*`) et un modèle spécifique au domaine pour le même `emailTemplateId`.
- Spécifier `domain` n'est pertinent que si vous avez différents domaines, ou souhaitez utiliser des modèles spécifiques pour les tests (`domain` défini sur `localhost` etc).
- Si vous spécifiez `domain`, il doit correspondre à un `DomainConfig`. En cas d'erreur, une liste de domaines valides est fournie.
- La syntaxe du modèle est EJS et est rendue avec un timeout de 500ms. Le P99 pour le rendu est <5ms, donc si vous atteignez 500ms, quelque chose ne va pas.
- **Votre modèle doit se rendre avec vos `testData` fournies** pour être enregistré. Les erreurs de rendu sont agrégées et rapportées dans le tableau de bord (bientôt disponible via l'API).

Les données minimales requises pour ajouter un modèle sont les suivantes :

[inline-code-attrs-start title = 'Exemple cURL Minimum de POST de EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

Vous voudrez peut-être avoir des modèles par site, auquel cas vous définissez `domain` :

[inline-code-attrs-start title = 'Exemple cURL de POST de EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête POST de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse POST de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The created template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]
