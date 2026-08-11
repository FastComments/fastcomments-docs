---
Suivez les mêmes étapes pour `localhost` comme vous le feriez en production. Assurez‑vous d'avoir configuré les domaines de production et les secrets d'API.

Tout d'abord, accédez à l'[Administration des Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). Ceci est accessible via Gérer les données -> Webhooks.

La page de configuration apparaît comme suit :

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Page d\'administration des Webhooks avec un sélecteur de domaine et un champ d\'URL de point de terminaison par événement de commentaire, plus Envoyer le payload de test'; title='Configuration des Webhooks'; cacheBuster = 'v3' app-screenshot-end]

Sur cette page, vous pouvez spécifier les points de terminaison pour chaque type d'événement de commentaire.

Pour chaque type d'événement, assurez‑vous de cliquer sur Envoyer le payload de test pour vous assurer que vous avez correctement configuré votre intégration. Voir la section suivante, "Testing", pour plus de détails.

---