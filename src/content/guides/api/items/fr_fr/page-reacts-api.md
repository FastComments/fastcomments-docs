Page Reacts permet à vos utilisateurs d’aimer une page, ou d’y réagir avec votre propre ensemble d’images de réaction. Le widget [Page Reacts](/guide-page-reacts.html) et le widget Floating Likes sont construits sur ces points de terminaison, et vous pouvez les appeler vous‑même pour créer votre propre bouton J’aime.

Contrairement au reste de ce guide, les points de terminaison Page Reacts sont publics. Ils sont appelés depuis les navigateurs de vos utilisateurs, ne nécessitent aucune clé API et ne coûtent aucun crédit API. Chaque réaction appartient à l’utilisateur qui effectue la requête, ainsi un utilisateur ne peut ajouter ou supprimer que ses propres réactions.

Il existe deux ensembles de points de terminaison :

- `/page-reacts/v1/likes/:tenantId` – un seul « like » par utilisateur et par page. Utilisez‑les pour un bouton J’aime.  
- `/page-reacts/v2/:tenantId` – plusieurs réactions par page, chacune identifiée par un court `id` que vous choisissez (par exemple `heart` ou `laugh`).

Les deux sont également disponibles dans nos SDKs dans le cadre du `PublicApi`, par exemple `getV1PageLikes`, `createV1PageReact` et `deleteV1PageReact` dans le [JavaScript SDK](/guide-sdk-javascript.html).

### Affichage des personnes ayant réagi

Passez `includeUserInfo=public` aux points de terminaison `GET` pour obtenir également les utilisateurs qui ont aimé ou réagi, avec leurs noms d’utilisateur et avatars, les plus récents en premier. Les utilisateurs qui ont rendu leur activité de profil privée sont exclus, tout comme les utilisateurs anonymes. Jusqu’à 100 réactions sont retournées.

Les utilisateurs SSO sont créés avec une activité de profil privée sauf si votre charge utile SSO définit `isProfileActivityPrivate` à `false`, donc définissez‑la pour les utilisateurs que vous souhaitez voir apparaître dans ces listes.

Sur les pages qui nécessitent SSO, seuls les utilisateurs SSO qui peuvent voir la page peuvent lister qui a réagi. Les autres reçoivent un `403` avec le code `sso-required` ou `page-forbidden`.

Depuis votre serveur, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) et [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) renvoient les mêmes listes avec une clé API, et acceptent également `includeUserInfo=include-private` pour inclure les utilisateurs avec des profils privés.

### Identifier l'utilisateur

Les réactions sont liées à l'utilisateur qui effectue la requête :

- **Utilisateurs SSO :** passez le paramètre de requête `sso`, défini sur le JSON encodé en URI du même objet SSO que vous fournissez au widget de commentaire. Voir [SSO](/guide-customizations-and-configuration.html#sso).  
- **Utilisateurs anonymes :** lorsqu’il n’y a pas de paramètre `sso` et aucune connexion FastComments, le serveur attribue au navigateur un identifiant anonyme stocké dans le cookie de session FastComments. Envoyez les requêtes avec `credentials: 'include'` afin que le cookie soit conservé entre les requêtes. Les navigateurs qui bloquent les cookies tiers ne conserveront pas l’identifiant anonyme, utilisez donc SSO lorsque chaque utilisateur doit être reconnu de façon fiable.

### L'urlId

`urlId` identifie la page, de la même manière que pour les commentaires. Utilisez le même `urlId` que vous fournissez au widget de commentaire afin que les likes et les commentaires soient comptés sur la même page. N’oubliez pas de l’encoder en URI.

[inline-code-attrs-start title = 'Exemple de bouton J\'aime'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optionnel, pour les utilisateurs SSO. Le même objet que vous fournissez à l'option "sso" du widget de commentaire.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]