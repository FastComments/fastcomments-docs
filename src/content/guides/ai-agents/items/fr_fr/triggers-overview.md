Un **déclencheur** est un événement qui réveille un agent. Chaque agent peut avoir un ou plusieurs déclencheurs définis.

### La liste complète

| Trigger | When it fires |
|---|---|
| [Commentaire ajouté](#trigger-comment-add) | Un nouveau commentaire est posté. |
| [Commentaire modifié](#trigger-comment-edit) | Un commentaire est modifié. Le texte précédent est inclus dans le contexte de l'agent. |
| [Commentaire supprimé](#trigger-comment-delete) | Un commentaire est supprimé. |
| [Commentaire épinglé](#trigger-comment-pin) | Un commentaire est épinglé (par n'importe qui, y compris un modérateur ou un autre agent). |
| [Commentaire désépinglé](#trigger-comment-unpin) | Un commentaire est désépinglé. |
| [Commentaire verrouillé](#trigger-comment-lock) | Un commentaire est verrouillé (aucune réponse supplémentaire autorisée). |
| [Commentaire déverrouillé](#trigger-comment-unlock) | Un commentaire est déverrouillé. |
| [Commentaire atteint le seuil de votes](#trigger-comment-vote-threshold) | Le nombre net de votes d'un commentaire atteint le seuil configuré. |
| [Commentaire atteint le seuil de signalements](#trigger-comment-flag-threshold) | Le nombre de signalements d'un commentaire atteint exactement le seuil configuré. |
| [Utilisateur publie son premier commentaire](#trigger-new-user-first-comment) | Un utilisateur publie son premier commentaire sur ce site. |
| [Commentaire auto-spam](#trigger-comment-auto-spammed) | Un commentaire est automatiquement signalé comme spam par le moteur anti-spam. |
| [Modérateur a revu le commentaire](#trigger-moderator-reviewed) | Un modérateur marque un commentaire comme revu. |
| [Modérateur approuve le commentaire](#trigger-moderator-approved) | Un modérateur approuve un commentaire. |
| [Modérateur marque comme spam](#trigger-moderator-spammed) | Un modérateur marque un commentaire comme spam. |
| [Modérateur attribue un badge](#trigger-moderator-awarded-badge) | Un modérateur attribue un badge à un utilisateur. |

### Plusieurs déclencheurs par agent

Un agent peut s'abonner à n'importe quelle combinaison de déclencheurs - le [Modèle de modérateur](#template-moderator) s'abonne par exemple à `COMMENT_ADD` et `COMMENT_FLAG_THRESHOLD`. Chaque événement déclenche l'agent une fois avec le contexte de cet événement.

### Ce qui empêche l'exécution d'un agent

Un événement de déclencheur auquel il est abonné **n'**exécute pas l'agent si l'une des conditions suivantes est vraie :

- Le [statut](#status-states) de l'agent est **Désactivé**.
- La [portée d'URL ou de locale](#scope-url-locale) de l'agent ne correspond pas au commentaire déclencheur.
- Le [budget journalier, mensuel ou de limitation de débit](#budgets-overview) de l'agent est épuisé - le déclencheur est enregistré comme **abandonné** avec une raison. Voir [Raisons d'abandon](#drop-reasons).
- La concurrence pour cet agent est saturée (plafonnée par agent).
- Le tenant de l'agent a une facturation invalide.
- L'action déclenchante a elle-même été effectuée par un bot ou un autre agent (prévention des boucles).
- Le déclencheur concernait un commentaire qui a déjà été traité par cet agent dans la fenêtre de déduplication.

Lorsqu'un déclencheur auquel il est abonné se déclenche avec succès, l'[Historique d'exécution](#run-history) de l'agent affiche une ligne avec le statut **Démarré** qui passe à **Succès** ou **Erreur** lorsque l'exécution se termine.

### Seuils de votes et de signalements

Deux déclencheurs - **Commentaire atteint le seuil de votes** et **Commentaire atteint le seuil de signalements** - nécessitent un seuil numérique sur le formulaire d'édition. Le déclencheur se déclenche au moment où le compte dépasse la valeur configurée (plus précisément, le déclencheur de seuil de signalement se déclenche lorsque `flagCount === flagThreshold`, donc choisir 1 signifie "se déclenche au premier signalement", et choisir 5 signifie "se déclenche à l'arrivée du cinquième signalement").

### Déclencheurs différés

Tout déclencheur peut être différé pour que l'agent s'exécute plus tard, par exemple après que les votes/signalements/réponses ont eu le temps de se stabiliser. Voir [Déclencheurs différés](#trigger-deferred-delay).

### Prévention des boucles

Pour empêcher les boucles infinies, les commentaires **écrits par un agent** portent un `botId`. Les déclencheurs de nouveau commentaire ignorent les commentaires avec un `botId`.

Effet net : les agents peuvent agir en réponse aux actions *humaines* dans votre tenant, mais les actions provenant d'agents ne déclenchent jamais aucun déclencheur d'agent. Cela s'applique à tous les types de déclencheurs.

### REPLAY : le déclencheur interne

Il existe également un type de déclencheur interne `REPLAY` utilisé par la fonctionnalité [Exécutions de test (Replays)](#test-runs-replays). Vous ne pouvez pas le sélectionner dans le formulaire d'édition - il existe pour que les exécutions de replay soient étiquetées distinctement dans l'historique d'exécution et exclues des vues d'exécution en direct.