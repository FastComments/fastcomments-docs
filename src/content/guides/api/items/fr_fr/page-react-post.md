[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Ajoute une réaction à une page en tant qu'utilisateur actuel. Un utilisateur peut ajouter chaque identifiant de réaction une fois : l'ajouter à nouveau réussit avec le code `already-reacted` et ne modifie pas le compteur. Un utilisateur peut ajouter plusieurs réactions différentes à la même page.

Les identifiants de réaction sont choisis par vous et peuvent comporter jusqu'à 36 caractères. La page est créée si elle n'existe pas encore. Passez `title` pour définir ou mettre à jour le titre de la page.

[inline-code-attrs-start title = 'Exemple cURL de réaction de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête de réaction de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** The reaction id, up to 36 characters. **/
    id: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse de réaction de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' when the user had already added this reaction. 'react-id-too-long' (HTTP 422) when the id is over 36 characters. Otherwise included on failure. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]