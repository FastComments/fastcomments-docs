Un **déclencheur** est un événement qui réveille un agent. Chaque agent peut avoir un ou plusieurs déclencheurs définis.

### La liste complète

| Trigger | Quand il se déclenche |
|---|---|
| [Comment Added](#trigger-comment-add) | Un nouveau commentaire est publié. |
| [Comment Edited](#trigger-comment-edit) | Un commentaire est modifié. Le texte précédent est inclus dans le contexte de l'agent. |
| [Comment Deleted](#trigger-comment-delete) | Un commentaire est supprimé. |
| [Comment Pinned](#trigger-comment-pin) | Un commentaire est épinglé (par n'importe qui, y compris un modérateur ou un autre agent). |
| [Comment Unpinned](#trigger-comment-unpin) | Un commentaire est désépinglé. |
| [Comment Locked](#trigger-comment-lock) | Un commentaire est verrouillé (aucune réponse supplémentaire autorisée). |
| [Comment Unlocked](#trigger-comment-unlock) | Un commentaire est déverrouillé. |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | Le nombre net de votes d'un commentaire atteint le seuil configuré. |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | Le nombre de signalements d'un commentaire atteint exactement le seuil configuré. |
| [User Posts First Comment](#trigger-new-user-first-comment) | Un utilisateur publie son premier commentaire sur ce site. |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | Un commentaire est automatiquement signalé comme spam par le moteur anti-spam. |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | Un modérateur marque un commentaire comme examiné. |
| [Moderator Approves Comment](#trigger-moderator-approved) | Un modérateur approuve un commentaire. |
| [Moderator Marks Spam](#trigger-moderator-spammed) | Un modérateur marque un commentaire comme spam. |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | Un modérateur attribue un badge à un utilisateur. |

### Plusieurs déclencheurs par agent

Un agent peut s'abonner à n'importe quelle combinaison de déclencheurs - le [modèle Modérateur](#template-moderator) s'abonne par exemple à la fois à `COMMENT_ADD` et `COMMENT_FLAG_THRESHOLD`. Chaque événement déclenche l'agent une fois avec le contexte de cet événement.

### Ce qui empêche le déclenchement d'un agent

Un événement de déclencheur auquel l'agent est abonné **ne** déclenche **pas** l'agent si l'une des conditions suivantes est remplie :

- Le [statut](#status-states) de l'agent est **Désactivé**.
- La [portée d'URL ou de locale](#scope-url-locale) de l'agent ne correspond pas au commentaire déclencheur.
- Le [budget journalier, mensuel ou de taux](#budgets-overview) de l'agent est épuisé - le déclencheur est enregistré comme **abandonné** avec une raison. Voir [Raisons d'abandon](#drop-reasons).
- La concurrence pour cet agent est saturée (plafonnée par agent).
- Le locataire de l'agent a une facturation invalide.
- L'action déclenchante a elle-même été effectuée par un bot ou un autre agent (prévention des boucles).
- Le déclencheur concernait un commentaire qui a déjà été traité par cet agent dans la fenêtre de déduplication.

Lorsqu'un déclencheur abonné se déclenche avec succès, l'[Historique d'exécution](#run-history) de l'agent affiche une ligne avec le statut **Démarré** qui passe à **Réussi** ou **Erreur** lorsque l'exécution se termine.

### Seuils de votes et de signalements

Deux déclencheurs - **Comment Crosses Vote Threshold** et **Comment Crosses Flag Threshold** - exigent un seuil numérique sur le formulaire de modification. Le déclencheur se déclenche au moment où le compteur franchit la valeur configurée (plus précisément, le déclencheur de seuil de signalement se déclenche quand `flagCount === flagThreshold`, donc choisir 1 signifie "déclencher au premier signalement", et choisir 5 signifie "déclencher à l'arrivée du cinquième signalement").

### Déclencheurs différés

Tout déclencheur peut être différé afin que l'agent s'exécute plus tard, par exemple après que les votes/signalements/réponses ont eu le temps de se stabiliser. Voir [Déclencheurs différés](#trigger-deferred-delay).

### Prévention des boucles

Pour prévenir les boucles infinies, les commentaires **écrits par un agent** portent un `botId`. Les déclencheurs de nouveau commentaire ignorent les commentaires avec un `botId`.

L'effet net : les agents peuvent agir en réponse aux actions *humaines* dans votre locataire, mais les actions provenant d'un agent ne déclenchent jamais de déclencheurs d'agent. Cela
s'applique à tous les types de déclencheurs.

### REPLAY : le déclencheur interne

Il existe également un type de déclencheur interne `REPLAY` utilisé par la fonctionnalité [Exécutions de test (Relectures)](#test-runs-replays). Vous ne pouvez pas le sélectionner sur le formulaire de modification - il existe afin que les exécutions de relecture soient étiquetées distinctement dans l'historique des exécutions et exclues des vues en direct.