[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Cette route fournit la capacité d'inviter un seul `Moderator`.

Les restrictions suivantes existent pour envoyer un email d'invitation à un `Moderator` :
- Le `Moderator` doit déjà exister.
- Le `fromName` ne peut pas dépasser `100 caractères`.

**Notes :**
- Si un utilisateur avec l'email fourni existe déjà, il sera invité à modérer les commentaires de votre locataire.
- Si un utilisateur avec l'email fourni **n'existe pas**, le lien d'invitation le guidera dans la création de son compte.
- L'invitation expirera après `30 jours`.

Nous pouvons créer un `Moderator` pour un utilisateur dont nous ne connaissons que l'email :

[inline-code-attrs-start title = 'Exemple cURL d\'Invitation de Modérateur'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Cela enverra un email comme `Bob de TenantName vous invite à être modérateur...`

[inline-code-attrs-start title = 'Structure de Requête d\'Invitation de Modérateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** The email sent to the user will appear to be sent from this name. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse d\'Invitation de Modérateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
