[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments est conçu pour être personnalisé. Le widget de commentaires lui-même s'exécute à l'intérieur d'une iframe pour des raisons de sécurité, donc pour appliquer
un style personnalisé vous devez suivre l'une des deux approches.

La première, la plus simple, et celle que nous privilégions, est d'utiliser la [page de personnalisation du widget](https://fastcomments.com/auth/my-account/customize-widget).

Dans la page de personnalisation du widget, consultez la section "Afficher les options avancées", sous laquelle se trouve une zone intitulée "Custom CSS" :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Cette approche présente plusieurs avantages :
1. Le CSS saisi est minifié avant d'être envoyé à l'utilisateur, et le formatage est maintenu de manière cohérente dans l'interface d'édition.
2. Vous bénéficiez de tous les avantages de l'interface de personnalisation du widget, par exemple la personnalisation facile du widget de commentaires différemment selon les sites.
3. Lorsque nous apportons des modifications au widget de commentaires, votre style personnalisé sera testé dans le cadre de notre processus de publication.

La seconde approche consiste à spécifier le paramètre **customCSS** dans la configuration du widget, comme suit :

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Cependant, cela présente des *limitations* :
1. Il existe une limite à la quantité de CSS personnalisé pouvant être transmise avant que nos serveurs ne rejettent la requête, en raison de la taille des en-têtes.
2. Vous devez gérer le CSS personnalisé dans votre infrastructure et votre système de build. Cela peut être un avantage plutôt qu'un inconvénient, selon le cas.
3. Il y a une surcharge supplémentaire d'envoyer le CSS personnalisé sur le réseau **deux fois** dans ce cas d'utilisation, puisqu'il doit être envoyé à nos serveurs, puis renvoyé dans le contenu de l'iframe. Cependant, pour la plupart des tailles de charge utile, cela n'est pas perceptible.
4. Une optimisation courante consiste à minifier le CSS pour réduire sa taille sur le réseau ; avec cette approche, vous devrez vous en charger.
5. Votre CSS personnalisé ne sera pas testé lorsque nous effectuerons des modifications.

### Fichiers CSS externes

Vous pouvez demander au widget de récupérer un fichier externe en utilisant `@import` !

Il est recommandé de placer le `@import` dans une règle de personnalisation. De cette façon, si nous devons un jour apporter une modification au widget de commentaires, nous pouvons utiliser nos outils d'automatisation
pour vérifier votre configuration. Par exemple, vous créeriez une règle de personnalisation dans l'interface de personnalisation du widget, cliqueriez sur `Advanced`, et entreriez dans `Custom CSS` :

    @import url(https://example.com/styles.css);

#### Dans le code - Non recommandé

Vous pouvez également charger un fichier CSS externe via la propriété `customCSS` :

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Cependant, rappelez-vous que votre CSS ne pourra pas être testé par nos soins si vous procédez ainsi. 

### Style du modal de profil utilisateur

Les modaux de profil utilisateur peuvent également être stylés avec du CSS personnalisé. Cependant, pour garantir que le style personnalisé soit appliqué aux profils utilisateur, tous les sélecteurs CSS doivent être préfixés par `.user-profile`. Sans ce préfixe, le style personnalisé sera ignoré pour les modaux de profil utilisateur.

Par exemple :

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Rétrocompatibilité

Chez FastComments, nous savons que nos clients personnalisent le widget de commentaires. C'est voulu — la dernière chose que nous souhaitons est que notre produit provoque des incohérences de design dans votre produit.

Comme il s'agit d'une partie importante de notre produit, nous avons une chaîne de build qui nous permet de réviser les modifications apportées au widget de commentaires, par client, à chaque version.

Si nous trouvons des problèmes mineurs, nous mettrons à jour votre compte pour assurer le bon déroulement de notre publication. Si nous détectons des changements majeurs et incompatibles, cela nous permet d'arrêter la publication.

---