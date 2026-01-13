Il existe plusieurs endpoints pour obtenir les comptages, selon ce que vous voulez et si vous souhaitez les obtenir depuis un navigateur, un serveur ou en utilisant le SDK API.

## Comptages Publics de Commentaires

Vous pouvez obtenir les comptages publics de commentaires en utilisant les widgets ci-dessus ou en utilisant les APIs qu'ils utilisent. Ces APIs sont restees inchangees depuis 2019 et ne changeront jamais.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Cela retournera une structure comme:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

La propriete `postfix` est toujours incluse.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Cela retournera une structure comme:

[inline-code-attrs-start title = 'Bulk Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

L'objet `counts` n'est rempli que pour les pages qui ont des comptages. La map `translations` est toujours presente car elle est utilisee pour le widget.

### Comportement des Endpoints Publics / Mise en Cache

Les endpoints publics ont un mecanisme de mise en cache de 60 secondes pour gerer les pics de trafic. En interne, il s'agit d'un cache LRU par thread en memoire sur le serveur, donc vous pouvez voir les comptages changer legerement (monter puis redescendre temporairement) lorsque les gens laissent beaucoup de commentaires.

Les endpoints publics retournent toujours le nombre *total* de commentaires, pas le nombre de commentaires racine.

### APIs Cote Serveur / SDK

La facon d'obtenir des commentaires depuis votre serveur est d'appeler l'[API Pages](/guide-api.html#page-structure) et d'obtenir l'objet page, qui contient le nombre total de commentaires et le nombre de commentaires racine. Nous fournissons des SDKs qui vous permettent d'appeler cette API sans construire manuellement la requete API et fournissent des valeurs de retour typees.
