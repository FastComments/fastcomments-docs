[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Cette route crée un seul `FeedPost`. Chaque post possède un auteur, donc `fromUserId` est requis et doit être l'identifiant d'un utilisateur FastComments ou SSO existant sur le compte.

[inline-code-attrs-start title = 'Exemple cURL de création FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de requête de création FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Pousse le post vers les flux qui sont ouverts dans un navigateur en ce moment. Valeur par défaut : false. **/
    isLive?: boolean
    /** Exécute le post à travers le moteur anti-spam avant de l'enregistrer. Valeur par défaut : false. **/
    doSpamCheck?: boolean
    /** Ignore la vérification de contenu répété qui s'exécute dans le cadre de doSpamCheck. Valeur par défaut : false. **/
    skipDupCheck?: boolean
    /** Jusqu'à 256 caractères. Renvoyé aux auditeurs en direct afin qu'un client puisse ignorer sa propre diffusion. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Obligatoire. Un identifiant d'utilisateur FastComments ou SSO. **/
    fromUserId: string
    title?: string
    /** HTML. Nettoyé lors de l'enregistrement. **/
    contentHTML?: string
    /** Remplace le nom d'affichage provenant de l'utilisateur. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de réponse de création FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Inclus en cas d'échec. **/
    reason?: string
    feedPost?: FeedPost; // Nous renvoyons le post complet créé en cas de succès.
}
[inline-code-end]