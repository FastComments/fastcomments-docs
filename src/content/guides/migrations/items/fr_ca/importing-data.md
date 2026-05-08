---
Bien que l’assistance de FastComments puisse aider pour les migrations, la plupart peuvent être effectuées et surveillées facilement sans aucune intervention
du personnel d’assistance.

Nous prenons en charge nativement l'importation des exports des fournisseurs suivants :

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

### Suivi des importations

FastComments utilise un système de traitement des tâches pour traiter les importations et exportations. Une fois que le système a pris en charge votre tâche, il
rapportera périodiquement l'état de la tâche dans l'interface d'importation ou d'exportation.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Notez que le statut des importations et des exportations est visible par tous les administrateurs du compte.

Si votre tâche échoue, elle ne sera pas redémarrée automatiquement. L'importation devra être tentée de nouveau. Si une importation ou une exportation échoue,
nos administrateurs système sont automatiquement avisés. Si nous identifions un problème, nous communiquerons avec vous pour voir si nous pouvons aider.

### Réexécution de l'importation

Lors de certaines migrations, il est nécessaire d'exécuter l'importation plusieurs fois. Par exemple, il est courant de faire une première passe
de migration pour des tests, puis de relancer l'importation avec les dernières données avant d'effectuer le basculement.

La réimportation du même contenu **ne créera pas de doublons**.

### Sécurité et expiration des données

Les fichiers d'importation ne sont accessibles d'aucune manière par des requêtes externes, et les fichiers d'importation sont supprimés de notre système dès que
l'importation est terminée.

---