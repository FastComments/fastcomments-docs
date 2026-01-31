Voici quelques symptômes que nous rencontrons fréquemment, ainsi que des solutions courantes. 

### "This is a demo" Message

Ceci s'affiche lorsque vous avez copié le code du widget depuis notre page d'accueil, qui utilise notre tenant de démonstration. Pour utiliser votre tenant, copiez le code du widget depuis [ici](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Error

FastComments doit connaître les domaines qui vous appartiennent afin d'authentifier les requêtes associées à votre compte. [Consultez notre documentation](/guide-multiple-sites.html#add-domains-to-account) pour voir comment résoudre cette erreur (ajoutez simplement sous-domaine + domaine exacts à votre compte).

Notez que cela ne devrait se produire qu'après la fin de la période d'essai. Pendant la période d'essai, toute requête provenant de nouveaux domaines sera automatiquement ajoutée à votre compte.

### Migrated Comments Not Showing for Custom Installations

Généralement, cela se produit lorsque les commentaires importés sont liés à un `Page ID`, et que vous transmettez une URL (ou aucune valeur, auquel cas elle prend par défaut l'URL de la page).

Vous pouvez déboguer cela en [exportant vos commentaires](https://fastcomments.com/auth/my-account/manage-data/export) et en regardant la colonne `URL ID` (actuellement la colonne `B`).

Assurez-vous que les valeurs que vous voyez dans la colonne `URL ID` sont les mêmes valeurs que vous transmettez à la configuration du widget en tant que paramètre `urlId`.

Pour plus d'explications, essayez de lire notre documentation [Comment les commentaires sont liés aux pages et aux articles](/guide-customizations-and-configuration.html#url-id).

Si tout échoue, [contactez-nous](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

Si le widget de commentaires ne s'affiche pas, vérifiez la console de développement de Chrome pour des erreurs.

Pour la plupart des mauvaises configurations, le widget de commentaires affichera au minimum une erreur sur la page s'il est capable de se charger. Ne rien voir est généralement une indication d'une erreur de script.

### Desired Configuration Not Working as Expected

Essayez notre [extension Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) pour voir quelle configuration est transmise au widget de commentaires. Si tout échoue, prenez une capture d'écran de ce que l'extension Chrome indique et [contactez-nous](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

Par défaut, FastComments utilisera l'URL de la page comme "compartiment" où les commentaires sont stockés. Si vos URLs incluent des `#hashbangs`, et que ces `#hashbangs` ne devraient pas faire partie de l'identifiant d'un fil de commentaires, nous pouvons simplement ignorer la valeur du hash bang, par exemple :

[inline-code-attrs-start title = 'Exemple : Ignorer les hash bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Notez qu'après avoir effectué ce changement, une migration devra être effectuée pour les commentaires existants. [Pour cela, contactez-nous.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

Par défaut, FastComments utilisera l'URL de la page comme "compartiment" où les commentaires sont stockés. Si vos URLs incluent des paramètres de requête qui ne devraient pas faire partie de l'identifiant d'un fil de commentaires, nous pouvons simplement les ignorer, par exemple :

[inline-code-attrs-start title = 'Ignorer les paramètres de requête'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Notez qu'après avoir effectué ce changement, une migration devra être effectuée pour les commentaires existants. [Pour cela, contactez-nous.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

Chez FastComments, nous mettons beaucoup d'efforts pour que la livraison de nos courriels soit aussi fiable que possible. Cependant, certains fournisseurs de messagerie sont notoirement difficiles à atteindre de manière fiable. Vérifiez votre dossier de pourriels pour les messages provenant de fastcomments.com.

Si vous [nous contactez](https://fastcomments.com/auth/my-account/help), nous pouvons généralement fournir plus d'informations sur les raisons pour lesquelles vous ne recevez pas nos courriels.