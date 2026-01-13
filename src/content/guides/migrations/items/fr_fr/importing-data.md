Bien que le support FastComments puisse aider pour les migrations, la plupart peuvent être effectuées et surveillées facilement sans
l'intervention du personnel de support.

Nous prenons en charge en natif l'importation des exports des fournisseurs suivants :

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via le plugin)

En vous rendant [ici](https://fastcomments.com/auth/my-account/manage-data/import) nous pouvons télécharger le fichier contenant les données à migrer.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Surveillance des imports

FastComments utilise un système de traitement par jobs pour traiter les imports et exports. Une fois que le système a pris en charge votre job, il
fera périodiquement remonter le statut du job dans l'interface d'import ou d'export.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Notez que le statut des imports et des exports est consultable par tous les administrateurs du compte.

Si votre job échoue, il ne sera pas automatiquement relancé. L'import devra être tenté à nouveau. Si un import ou un export échoue,
nos administrateurs système sont automatiquement notifiés. Si nous identifions un problème, nous vous contacterons pour voir si nous pouvons aider.

### Relancer l'import

Lors de certaines migrations, il est nécessaire d'exécuter l'import plusieurs fois. Par exemple, il est courant d'effectuer une première passe
de migration pour les tests, puis de relancer l'import avec les dernières données avant de basculer.

La réimportation du même contenu **ne créera pas de doublons**.

### Sécurité des données et expiration

Les fichiers d'import ne sont accessibles d'aucune manière via des requêtes externes, et les fichiers d'import sont supprimés de notre système dès que
l'import est terminé.