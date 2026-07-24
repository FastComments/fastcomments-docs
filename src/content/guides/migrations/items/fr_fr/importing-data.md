---
Bien que le support FastComments puisse aider avec les migrations, la plupart peuvent être effectuées et surveillées facilement sans aucune intervention du personnel de support.

Nous prenons en charge nativement l'importation d'exports des fournisseurs suivants :

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via le plugin)
- AnyComment (Via WordPress Import/Export)

En naviguant [ici](https://fastcomments.com/auth/my-account/manage-data/import), vous pouvez télécharger le fichier contenant les données à migrer.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='Le formulaire de la page d\'importation' app-screenshot-end]

### Surveillance des importations

FastComments utilise un système de traitement des tâches pour gérer les importations et les exportations. Une fois que le système a récupéré votre tâche, il rapporte périodiquement l'état de la tâche dans l'interface d'importation ou d'exportation.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Statut du travail d\'importation' app-screenshot-end]

Notez que le statut des importations et des exportations est visible par tous les administrateurs du compte.

Si votre tâche échoue, elle ne sera pas redémarrée automatiquement. L'importation devra être réessayée. Si une importation ou une exportation échoue, nos administrateurs système sont automatiquement notifiés. Si nous identifions un problème, nous vous contacterons pour voir si nous pouvons aider.

### Relancer l'importation

Lors de certaines migrations, il est nécessaire d'exécuter l'importation plusieurs fois. Par exemple, il est courant de faire une première passe de migration à des fins de test, puis de relancer l'importation avec les dernières données avant de basculer.

Réimporter le même contenu **ne créera pas de doublons**.

### Sécurité des données et expiration

Les fichiers d'importation ne sont pas accessibles par des requêtes externes de quelque manière que ce soit, et les fichiers d'importation sont supprimés de notre système dès que l'importation est terminée.

---