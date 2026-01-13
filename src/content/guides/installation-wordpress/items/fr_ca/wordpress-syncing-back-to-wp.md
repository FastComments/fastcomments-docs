---
Par défaut, FastComments synchronise quotidiennement avec votre site WordPress. Ceci sert uniquement de sauvegarde afin que vous continuiez de posséder une copie des données, et pour les extensions qui pourraient en dépendre.

Cela n'est pas effectué immédiatement pour chaque commentaire enregistré en raison du fait que certains sites, capables de gérer un fort trafic en lecture, ont des déploiements de base de données qui ne sont pas toujours capables de gérer un fort trafic en écriture (d'où le déchargement de ce travail vers FastComments).

La planification de la synchronisation vers WordPress peut être personnalisée en installant un plugin. Nous recommandons [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Installez WP Crontrol
2. Allez à `Settings -> Cron Schedules`.
3. Allez à l'onglet `Cron Events`.
4. Recherchez `fastcomments_cron_hook`.
5. Modifiez l'événement. Vous pouvez configurer le hook pour s'exécuter toutes les heures, deux fois par jour, quotidiennement (par défaut), ou une fois par semaine.

La synchronisation vers WordPress peut également être effectuée manuellement à tout moment en allant au tableau de bord du plugin FastComments et en sélectionnant `Manually Sync`. Vous aurez l'option de synchroniser vers votre installation WP, ou de téléverser à nouveau vos commentaires WP vers les serveurs de FastComments.

---