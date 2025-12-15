Voici quelques symptômes fréquemment rencontrés et leurs solutions courantes.

### Message « This is a demo »

Ceci s'affiche lorsque vous avez copié le code du widget depuis notre page d'accueil, qui utilise notre locataire de démonstration. Pour utiliser votre locataire, copiez le code du widget depuis [ici](https://fastcomments.com/auth/my-account/get-acct-code).

### Erreur « FastComments cannot load on this domain »

FastComments doit savoir quels domaines vous appartiennent pour authentifier les requêtes associées à votre compte. [Consultez notre documentation](/guide-multiple-sites.html#add-domains-to-account) pour voir comment résoudre cette erreur (ajoutez simplement le sous-domaine exact + le domaine à votre compte).

Notez que cela ne devrait se produire qu'après la fin de la période d'essai. Pendant la période d'essai, toutes les requêtes provenant de nouveaux domaines seront automatiquement ajoutées à votre compte.

### Les commentaires migrés ne s'affichent pas pour les installations personnalisées

Habituellement, cela se produit lorsque les commentaires importés sont liés à un `Page ID`, et que vous passez une URL (ou aucune valeur, auquel cas elle prend par défaut l'URL de la page).

Vous pouvez déboguer cela en [exportant vos commentaires](https://fastcomments.com/auth/my-account/manage-data/export) et en visualisant la colonne `URL ID` (actuellement la colonne `B`).

Assurez-vous que les valeurs que vous voyez dans la colonne `URL ID` sont les mêmes valeurs que vous passez à la configuration du widget comme paramètre `urlId`.

Pour plus d'explications, essayez de lire notre [documentation sur comment les commentaires sont liés aux pages et articles](/guide-customizations-and-configuration.html#url-id).

Si tout échoue, [contactez-nous](https://fastcomments.com/auth/my-account/help).

### Le widget de commentaires ne s'affiche pas

Si le widget de commentaires ne s'affiche pas, vérifiez la console développeur de Chrome pour les erreurs.

Pour la plupart des mauvaises configurations, le widget de commentaires affichera au moins une erreur sur la page s'il est capable de se charger. Ne rien voir est généralement une indication d'une erreur de script.

### La configuration souhaitée ne fonctionne pas comme prévu

Essayez notre [extension Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) pour voir quelle configuration est passée au widget de commentaires. Si tout échoue, prenez une capture d'écran de ce que l'extension Chrome indique et [contactez-nous](https://fastcomments.com/auth/my-account/help).

### Commentaires manquants sur la même URL avec un hash bang différent

Par défaut, FastComments utilisera l'URL de la page pour le « bucket » où les commentaires sont stockés. Si vos URL incluent des `#hashbangs`, et que ces `#hashbangs` ne devraient pas faire partie de l'identifiant qui identifie un fil de commentaires, nous pouvons simplement ignorer la valeur du hash bang, par exemple :

[inline-code-attrs-start title = 'Ignore Hash Bangs Example'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Notez qu'après avoir effectué ce changement, une migration devra être effectuée pour les commentaires existants. [Pour cela, contactez-nous.](https://fastcomments.com/auth/my-account/help)

### Les paramètres de requête URL affectent le widget

Par défaut, FastComments utilisera l'URL de la page pour le « bucket » où les commentaires sont stockés. Si vos URL incluent des paramètres de requête qui ne devraient pas faire partie de l'identifiant qui identifie un fil de commentaires, nous pouvons simplement les ignorer, par exemple :

[inline-code-attrs-start title = 'Ignore Query Parameters'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Notez qu'après avoir effectué ce changement, une migration devra être effectuée pour les commentaires existants. [Pour cela, contactez-nous.](https://fastcomments.com/auth/my-account/help)

### Non-réception des courriels

Chez FastComments, nous mettons beaucoup d'efforts pour assurer que notre livraison de courriels soit aussi fiable que possible. Cependant, certains fournisseurs de courriels sont notoirement difficiles à atteindre de manière fiable. Vérifiez votre dossier spam pour les messages de fastcomments.com.

Si vous [nous contactez](https://fastcomments.com/auth/my-account/help), nous pouvons généralement fournir plus d'informations sur pourquoi vous ne recevez peut-être pas nos courriels.
