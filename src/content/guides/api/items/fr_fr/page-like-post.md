---
[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Aime une page en tant qu'utilisateur actuel. Chaque utilisateur ne peut aimer une page qu'une fois : aimer à nouveau réussit avec le code `already-liked` et ne modifie pas le compteur.

La page est créée si elle n'existe pas encore. Passez `title` pour définir ou mettre à jour le titre de la page.

[inline-code-attrs-start title = 'Exemple cURL de Like de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête de Like de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Définit le titre de la page. **/
    title?: string
    /** JSON encodé en URI de votre objet SSO. Omettre pour les utilisateurs anonymes. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse de Like de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' lorsque l'utilisateur a déjà aimé la page. Sinon inclus en cas d'échec. **/
    code?: 'already-liked' | string
    /** Inclus en cas d'échec. **/
    reason?: string
}
[inline-code-end]

---