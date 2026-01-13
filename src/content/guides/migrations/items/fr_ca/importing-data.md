---
Bien que le soutien de FastComments puisse aider lors de migrations, la plupart peuvent être effectuées et surveillées facilement sans aucune intervention
du personnel de soutien.

Nous prenons en charge nativement l'importation des fichiers d'exportation des fournisseurs suivants :

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via le plugin)

En naviguant [ici](https://fastcomments.com/auth/my-account/manage-data/import) nous pouvons téléverser le fichier contenant les données à migrer.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitoring Imports

FastComments utilise un système de traitement par tâches pour traiter les importations et exportations. Une fois que le système a pris en charge votre tâche, il indiquera périodiquement le statut de la tâche dans l'interface d'importation ou d'exportation.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Notez que le statut des importations et des exportations est consultable par tous les administrateurs du compte.

Si votre tâche échoue, elle ne sera pas redémarrée automatiquement. L'importation devra être retentée. Si une importation ou une exportation échoue,
nos administrateurs système sont automatiquement avisés. Si nous identifions un problème, nous vous contacterons pour voir si nous pouvons aider.

### Re-Running The Import

Lors de certaines migrations, il est nécessaire d'exécuter l'importation plusieurs fois. Par exemple, il est courant d'effectuer une première passe
de migration à des fins de test, puis de relancer l'importation avec les données les plus récentes avant de procéder au basculement.

Réimporter le même contenu **ne créera pas de doublons**.

### Data Security and Expiration

Les fichiers d'importation ne sont en aucun cas accessibles via des requêtes externes, et les fichiers d'importation sont supprimés de notre système dès que
l'importation est terminée.

---