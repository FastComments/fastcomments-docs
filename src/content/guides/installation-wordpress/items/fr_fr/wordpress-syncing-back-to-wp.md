Par défaut, FastComments synchronise quotidiennement avec votre site WordPress. Ceci sert uniquement à des fins de sauvegarde afin que vous conserviez une copie des données, et pour les plugins
qui peuvent en dépendre.

Cela ne se produit pas immédiatement à chaque commentaire enregistré en raison du fait que certains sites, capables de gérer un fort trafic en lecture, n'ont pas toujours des déploiements de base de données capables de supporter un fort trafic en écriture (d'où le délestage de ce travail vers FastComments).

La planification de la synchronisation vers WordPress peut être personnalisée en installant un plugin. Nous recommandons [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Installez WP Crontrol
2. Allez dans `Settings -> Cron Schedules`.
3. Allez dans l'onglet `Cron Events`.
4. Recherchez `fastcomments_cron_hook`.
5. Modifiez l'événement. Vous pouvez configurer le hook pour s'exécuter toutes les heures, deux fois par jour, quotidiennement (par défaut), ou une fois par semaine.

La synchronisation vers WordPress peut également être effectuée manuellement à tout moment en allant sur le tableau de bord du plugin FastComments et en sélectionnant `Manually Sync`. Vous aurez
l'option de synchroniser vers votre installation WP, ou de téléverser à nouveau vos commentaires WP vers les serveurs de FastComments.