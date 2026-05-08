Bien que l'assistance FastComments puisse aider lors des migrations, la plupart peuvent être effectuées et surveillées facilement sans aucune intervention
du personnel d'assistance.

Nous prenons en charge nativement l'importation d'exports depuis les fournisseurs suivants :

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via le plugin)
- AnyComment (Via WordPress Import/Export)

En naviguant [ici](https://fastcomments.com/auth/my-account/manage-data/import) nous pouvons téléverser le fichier contenant les données à migrer.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitoring Imports

FastComments utilise un système de traitement par tâches pour traiter les importations et exportations. Une fois que le système a pris en charge votre tâche, il rapportera périodiquement l'état de la tâche dans l'interface d'importation ou d'exportation.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Notez que le statut des importations et des exportations est consultable par tous les administrateurs du compte.

Si votre tâche échoue, elle ne sera pas relancée automatiquement. L'importation devra être tentée à nouveau. Si une importation ou une exportation échoue,
nos administrateurs système sont automatiquement notifiés. Si nous identifions un problème, nous vous contacterons pour voir si nous pouvons aider.

### Re-Running The Import

Lors de certaines migrations, il est nécessaire d'exécuter l'importation plusieurs fois. Par exemple, il est courant d'effectuer une première migration de test, puis de relancer l'importation avec les données les plus récentes avant de basculer.

La réimportation du même contenu **ne créera pas de doublons**.

### Data Security and Expiration

Les fichiers d'importation ne sont accessibles d'aucune manière via des requêtes externes, et ils sont supprimés de notre système dès que l'importation est terminée.