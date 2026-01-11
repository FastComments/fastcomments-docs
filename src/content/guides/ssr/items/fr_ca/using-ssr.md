Pour utiliser FastComments SSR, le client peut récupérer du HTML à partir du point de terminaison `https://fastcomments.com/ssr/comments`.

Cela peut être fait de plusieurs manières.

### Avec WordPress

Le SSR est activé par défaut pour les utilisateurs sans JavaScript en tant que solution de repli dans le plugin WordPress depuis la version `3.10.2`.

### Sur une page Web

Pour une application existante, le SSR peut être ajouté avec l'[exemple suivant](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), en supposant que le langage utilisé est PHP :

[inline-code-attrs-start title = 'Exemple de SSR basé sur PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

Nous pouvons également n'afficher l'interface SSR que lorsque l'utilisateur a désactivé JS :

[inline-code-attrs-start title = 'Exemple de repli SSR basé sur PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

Pour un exemple utilisant SSO, [voir ici](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Avec du contenu pré-rendu

Notre blog est généré au moment de la génération, et fournit [un bon exemple de SSR avec Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Paramètres de base

Les paramètres de base que vous devez transmettre sont :
- `tenantId` - Ceci vous identifie en tant que client.
- `urlId` - Identifie la page ou l'article pour lequel charger les commentaires, et définit où ils sont sauvegardés.
- `url` - Utilisé pour les notifications et les fonctionnalités associées afin de renvoyer vers le fil de commentaires.

### Style personnalisé

La version SSR du widget de commentaires utilise la même structure et le même moteur de rendu que celle JavaScript.

Ainsi, tous les styles personnalisés qui fonctionnent pour le widget de commentaires JavaScript fonctionnent pour le SSR. 

### Remarques

Avec le SSR, il n'y a pas de JavaScript pour contrôler la hauteur du conteneur rendu. Dans les navigateurs, une barre de défilement verticale peut apparaître pour les discussions longues.

Vous devez donc ajuster cela selon vos besoins.

---