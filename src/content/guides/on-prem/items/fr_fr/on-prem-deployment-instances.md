### Composants requis

Pour On-Prem, FastComments se compose simplement d'un serveur d'application et d'une base de données. Nous avons simplifié le déploiement afin que
l'application puisse servir tout le trafic directement sans ajouter d'autres composants.

Le serveur d'application est fourni dans une image Docker et peut être déployé avec n'importe quelle solution de gestion de conteneurs.

La base de données, MongoDB, peut être auto-hébergée ou hébergée par un autre fournisseur comme AWS DocumentDB ou MongoDB Atlas.

FastComments est actuellement testé avec MongoDB 7, toutefois nous visons la compatibilité avec DocumentDB pour faciliter le déploiement.

### Tailles des instances

Vous verrez que FastComments est assez bien optimisé et ne nécessite pas de machines puissantes pour l'application elle-même afin de maintenir des P99 faibles.

Toutes les tâches par lot et les cron jobs utilisent le streaming pour limiter l'utilisation totale de mémoire.

Les tableaux ci‑dessous pour le serveur d'application et la base de données peuvent aider au dimensionnement.

### Instances du serveur d'application


| Utilisateurs simultanés | CPUs totaux du cluster | Mémoire totale du cluster |
|-------------------------|------------------------|---------------------------|
| 100                     | 1                      | 256mb                     |
| 1K                      | 2                      | 512mb                     |
| 10K                     | 8                      | 1gb                       |
| 100K                    | 32                     | 8gb                       |
| 1M                      | 64                     | 64gb                      |

Par exemple, un seul cœur servant environ 100 fils de discussion de commentaires par seconde n'utilise généralement jamais plus de 250mb RSS.

### Instances du serveur de base de données

Le dimensionnement de la base de données dépend de la taille du working set, qui est la quantité de données auxquelles vous accédez à un moment donné, ainsi que des requêtes simultanées.

FastComments est assez conciliant avec Mongo : pour les requêtes chaudes il utilise des index hints, des streaming cursors, et applique des limites de concurrence dans plusieurs zones
pour empêcher la surcharge des systèmes en aval.

Ce qui suit est une ligne directrice générale sur les tailles d'instances de base de données. **Notez que ceci est __par instance__, et non les ressources totales du cluster**.

| Utilisateurs simultanés | Commentaires stockés | CPUs par instance | Mémoire par instance |
|------------------------|----------------------|-------------------|----------------------|
| 100                    | 1k                   | 1                 | 256mb                |
| 1K                     | 5k                   | 2                 | 512mb                |
| 10K                    | 100k                 | 8                 | 2gb                  |
| 100K                   | 500k                 | 16                | 8gb                  |
| 1M                     | 5M                   | 32                | 32gb                 |

Les tableaux ci‑dessus sont des estimations prudentes. Vous constaterez peut-être que les besoins réels diffèrent selon votre configuration spécifique (tailles de page, volume de commentaires, etc).