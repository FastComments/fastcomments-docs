Suivez les mêmes étapes pour `localhost` que pour l'environnement de production. Assurez-vous d'avoir configuré les domaines de production et les API Secrets.

Tout d'abord, rendez-vous dans la [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Ceci est accessible via Manage Data -> Webhooks.

La page de configuration apparaît comme suit :

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

Sur cette page, vous pouvez spécifier des endpoints pour chaque type d'événement de commentaire.

Pour chaque type d'événement, assurez-vous de cliquer sur Send Test Payload pour vérifier que votre intégration est correctement configurée. Voir la section suivante, "Testing", pour plus de détails.