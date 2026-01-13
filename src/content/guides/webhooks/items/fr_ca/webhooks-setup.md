Suivez les mêmes étapes pour `localhost` que pour la production. Assurez-vous d'avoir configuré les domaines de production et les API Secrets.

First, navigate to the [Administration des webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Gérer les données -> Webhooks.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

Sur cette page, vous pouvez spécifier des endpoints pour chaque type d'événement de commentaire.

Pour chaque type d'événement, assurez-vous de cliquer sur Envoyer la charge utile de test pour vérifier que votre intégration est correctement configurée. Voir la section suivante, «Tests», pour plus de détails.