---
Les webhooks d'agent sont réessayés en cas d'échec. La livraison est **en mode « fire-and-forget » du point de vue de l'agent** - une livraison échouée n'empêche pas l'exécution de l'agent ni n'annule des actions - et une file d'attente + cron gèrent les réessais de manière asynchrone.

### Modèle de file d'attente

Chaque événement est mis en file **une fois par webhook correspondant**. Ainsi, si vous avez trois webhooks abonnés à `trigger.succeeded` pour un agent + domaine donnés, la plateforme met en file trois livraisons ; chacune est livrée et réessayée indépendamment. L'échec d'un webhook n'affecte jamais les autres.

### Ce qui est réessayé

Une livraison est réessayée lorsque :

- La requête HTTP **ne s'achève pas** (échec DNS, connexion refusée, timeout).
- Le code de réponse HTTP est un statut non-2xx qui n'est pas dans la liste configurée des **Codes d'état sans réessai**.

Une livraison n'est **pas réessayée** lorsque :

- Le code de réponse est `2xx` (succès).
- Le code de réponse figure dans la liste configurée des **Codes d'état sans réessai**. Par défaut cette liste est vide - tout statut non-2xx est réessayé.

### Configuration des codes sans réessai

Le formulaire de configuration du webhook comporte un champ **Codes d'état sans réessai** (valeurs multiples). Entrées courantes :

- `410` - Gone. Votre endpoint est déplacé de façon permanente ou la ressource a disparu. Réessayer ne fait que gaspiller la bande passante des deux côtés.
- `422` - Unprocessable Entity. Votre endpoint a compris la payload mais l'a considérée invalide. Réessayer avec la même payload donnera la même réponse.
- `400` - Bad Request, dans le même esprit.

Ajouter un code ici signifie : lorsque l'endpoint le retourne, marquer la livraison comme failed-terminal et arrêter les réessais.

### Planification des réessais

Un worker en arrière-plan s'exécute toutes les quelques secondes et traite les livraisons dont le délai avant la prochaine tentative est dépassé.

Après chaque échec, l'heure de la prochaine tentative est repoussée selon un **backoff linéaire** : le délai augmente selon `60 seconds * attempt count` (donc la tentative 1 attend 1 minute, la tentative 2 attend 2 minutes, et ainsi de suite).

Après 99 tentatives échouées (ou 3 en développement local), la livraison est abandonnée et supprimée de la file d'attente. Les entrées du journal de livraison persistent et restent visibles dans la page [Journaux de livraison des webhooks](#webhook-logs) jusqu'à expiration.

### Idempotence de votre côté

Parce que nous réessayons, votre endpoint **doit être idempotent**. Le même `triggerId` (ou `approvalId`) peut arriver plusieurs fois. Votre endpoint devrait :

- Utiliser une clé unique (`triggerId` pour les événements trigger, `approvalId` pour les événements approval) comme token de déduplication.
- Accepter les livraisons en double gracieusement (retourner 200 la deuxième fois).

Un endpoint non idempotent finira par traiter en double certaines livraisons, notamment lors de pannes transitoires où un timeout provoque un réessai 30 secondes plus tard alors que la requête initiale avait en réalité réussi.

### Ordre

Les livraisons ne sont **pas strictement ordonnées**. Un `trigger.succeeded` et un `approval.requested` en aval (du même run) peuvent arriver dans n'importe quel ordre si l'un est réessayé et pas l'autre. Votre endpoint ne doit pas supposer un ordre causal.

Si vous avez besoin d'ordre, utilisez les horodatages - `occurredAt` dans l'enveloppe, ainsi que le `createdAt` du trigger/approval dans le bloc data - pour reconstruire l'ordre de votre côté.

### Nettoyage

Les livraisons sont retirées de la file d'attente dès qu'elles réussissent ou atteignent le plafond de tentatives. La plateforme ne conserve pas les livraisons échouées de façon terminale dans la file elle-même ; l'enregistrement durable de chaque tentative se trouve dans la page [Journaux de livraison des webhooks](#webhook-logs).

### Où regarder lorsque les réessais échouent

La page [Journaux de livraison des webhooks](#webhook-logs) est l'endroit pour voir pourquoi un webhook échoue. Causes courantes :

- **Échec de résolution DNS** - l'URL est incorrecte ou le domaine a disparu.
- **Erreurs TLS** - le certificat de votre endpoint est invalide ou expiré.
- **Connexion refusée / timeout** - votre endpoint est hors service.
- **Réponses 5xx** - votre endpoint est en ligne mais a généré une erreur. Le corps de la réponse (tronqué) est enregistré.
- **Réponses 4xx** - votre endpoint a rejeté la payload. Si c'est intentionnel, ajoutez le code aux **Codes d'état sans réessai**.

### Mettre en pause un webhook défaillant

Si un webhook échoue de manière persistante, la solution la plus propre est de le supprimer (ou de vider temporairement sa liste d'abonnement aux événements). La plateforme ne désactive pas automatiquement les webhooks en échec - ils continuent de réessayer jusqu'à l'abandon de la livraison.

---