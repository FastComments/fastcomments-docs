Suivez les mêmes étapes pour `localhost` comme vous le feriez en production. Assurez-vous d'avoir configuré les domaines de production et les API Secrets.

First, navigate to the [administration des Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Manage Data -> Webhooks.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

Sur cette page, vous pouvez spécifier des points de terminaison pour chaque type d'événement de commentaire.

Pour chaque type d'événement, assurez-vous de cliquer sur Envoyer la charge utile de test afin de vérifier que votre intégration est correctement configurée. Voir la section suivante, « Tests », pour plus de détails.