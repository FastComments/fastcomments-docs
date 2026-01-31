Voici quelques symptômes que nous rencontrons fréquemment, et des solutions courantes. 

### "Ceci est une démo" Message

Ceci s'affiche lorsque vous avez copié le code du widget depuis notre page d'accueil, qui utilise notre tenant de démonstration. Pour utiliser votre tenant, copiez le code du widget depuis [ici](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Erreur

FastComments a besoin de savoir quels domaines vous appartiennent pour authentifier les requêtes associées à votre compte. [Consultez notre documentation](/guide-multiple-sites.html#add-domains-to-account) pour voir comment résoudre cette erreur (ajoutez simplement le sous-domaine + domaine exact à votre compte).

Notez que cela ne devrait se produire qu'après la fin de la période d'essai. Pendant la période d'essai, toutes les requêtes provenant de nouveaux domaines seront automatiquement ajoutées à votre compte.

### Les commentaires migrés n'apparaissent pas pour les installations personnalisées

Généralement, cela se produit lorsque les commentaires importés sont liés à un `Page ID`, et que vous passez une URL (ou aucune valeur, auquel cas elle prend par défaut l'URL de la page).

Vous pouvez déboguer cela en [exportant vos commentaires](https://fastcomments.com/auth/my-account/manage-data/export) et en regardant la colonne `URL ID` (actuellement la colonne `B`).

Assurez-vous que les valeurs que vous voyez dans la colonne `URL ID` sont les mêmes valeurs que vous fournissez à la configuration du widget en tant que paramètre `urlId`.

Pour plus d'explications, essayez de lire notre documentation [Comment les commentaires sont liés aux pages et articles](/guide-customizations-and-configuration.html#url-id).

Si tout échoue, [contactez-nous](https://fastcomments.com/auth/my-account/help).

### Le widget de commentaires n'apparaît pas

Si le widget de commentaires n'apparaît pas, vérifiez la console développeur de Chrome pour voir les erreurs.

Pour la plupart des mauvaises configurations, le widget de commentaires affichera au moins une erreur sur la page s'il parvient à se charger. Ne rien voir est généralement une indication d'une erreur de script.

### La configuration souhaitée ne fonctionne pas comme attendu

Essayez notre [extension Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) pour voir quelle configuration est passée au widget de commentaires. Si tout échoue, prenez une capture d'écran de ce que l'extension Chrome indique et [contactez-nous](https://fastcomments.com/auth/my-account/help).

### Commentaires manquants sur la même URL avec un hashbang différent

Par défaut, FastComments utilisera l'URL de la page pour le « compartiment » où les commentaires sont stockés. Si vos URLs incluent des `#hashbangs`, et que ces `#hashbangs`
ne doivent pas faire partie de l'identifiant qui détermine un fil de commentaires, nous pouvons simplement ignorer la valeur du hash bang, par exemple :

[inline-code-attrs-start title = 'Exemple : Ignorer les hashbangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Notez qu'après avoir effectué ce changement, une migration devra être réalisée pour les commentaires existants. [Pour cela, contactez-nous.](https://fastcomments.com/auth/my-account/help)

### Paramètres de requête URL affectant le widget

Par défaut, FastComments utilisera l'URL de la page pour le « compartiment » où les commentaires sont stockés. Si vos URLs incluent des paramètres de requête
qui ne doivent pas faire partie de l'identifiant qui détermine un fil de commentaires, nous pouvons simplement les ignorer, par exemple :

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

Notez qu'après avoir effectué ce changement, une migration devra être réalisée pour les commentaires existants. [Pour cela, contactez-nous.](https://fastcomments.com/auth/my-account/help)

### Ne pas recevoir les e-mails

Chez FastComments, nous mettons beaucoup d'efforts pour que la livraison de nos e-mails soit aussi fiable que possible. Cependant, certains fournisseurs de messagerie sont notoirement difficiles à atteindre de manière fiable. Vérifiez votre dossier spam pour des messages provenant de fastcomments.com.

Si vous [nous contactez](https://fastcomments.com/auth/my-account/help), nous pouvons généralement fournir
plus d'informations sur la raison pour laquelle vous ne recevez peut-être pas nos e-mails.

---