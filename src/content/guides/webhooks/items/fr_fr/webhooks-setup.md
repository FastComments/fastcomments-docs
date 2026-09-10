---
Suivez les mêmes étapes pour `localhost` que pour la production. Assurez‑vous d'avoir configuré les domaines de production et les secrets d'API.

Tout d'abord, accédez à l'[Administration des Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). Elle est accessible via Gérer les données -> Webhooks.

La page répertorie chaque webhook de votre compte :

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Page d\'administration des Webhooks répertoriant chaque webhook avec son URL, son événement, son domaine, sa méthode, son statut et le nombre d\'événements en file d\'attente'; title='Liste des Webhooks'; cacheBuster = 'v4' app-screenshot-end]

Cliquez sur **Nouveau Webhook** pour en ajouter un. Chaque webhook possède une URL, un événement de commentaire (créé, mis à jour ou supprimé), un domaine et une méthode HTTP :

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Formulaire de nouveau webhook avec les champs URL, événement, domaine et méthode HTTP ainsi que le bouton Envoyer une charge utile de test'; title='Nouveau Webhook'; cacheBuster = 'v4' app-screenshot-end]

Chaque webhook est livré de manière indépendante. Vous pouvez envoyer le même événement à plusieurs points de terminaison, et un webhook limité à **Tous les domaines** reçoit les commentaires de chaque domaine même lorsqu'un webhook spécifique à un domaine existe pour le même événement. La même URL, le même événement et le même domaine ne peuvent pas être ajoutés deux fois.

Avant d'enregistrer, cliquez sur **Envoyer une charge utile de test** pour vérifier que le point de terminaison accepte une requête signée. Consultez la section suivante, "Testing", pour plus de détails.

Depuis la liste, vous pouvez modifier, désactiver, réactiver ou supprimer un webhook. La désactivation conserve les événements en file d'attente jusqu'à ce que le webhook soit réactivé ; la suppression les supprime.

Les webhooks peuvent également être créés via l'API, par exemple avec Zapier. Ils apparaissent dans la même liste avec la source **API**. Voir la gestion des webhooks via l'API.

---