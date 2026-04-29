Depuis la [page Agents IA](https://fastcomments.com/auth/my-account/ai-agents) vous pouvez créer un agent de deux façons :

- **À partir d’un modèle.** Cliquez sur **Parcourir les modèles** et choisissez l’un des quatre agents de démarrage intégrés. Le formulaire arrive pré-rempli et le statut de l’agent est **Exécution à blanc**. Voir [Modèles de démarrage](#starter-templates).
- **Depuis zéro.** Cliquez sur **Créer un nouvel agent**. Le formulaire est vierge.

Dans les deux cas, c’est le même formulaire d’édition que vous sauvegardez et modifiez ensuite. Cette page parcourt le formulaire de haut en bas.

### Notions de base

- **Nom interne.** Un identifiant court utilisé uniquement dans les tableaux de bord d’administration (historique des exécutions, analytics, journaux d’audit). Les minuscules avec underscores fonctionnent bien : `moderator`, `welcome_greeter`. Si le nom interne d’un modèle est déjà pris, le formulaire ajoute automatiquement un suffixe (`tos_enforcer_2`, etc.).
- **Nom affiché.** Affiché publiquement chaque fois que l’agent publie un commentaire. C’est ce que voient vos lecteurs.
- **Statut.** Désactivé, Exécution à blanc ou Activé. Les nouveaux agents sont toujours par défaut en Exécution à blanc. Voir [États du statut](#status-states).

### Modèle

Choisissez le LLM. Voir [Choisir un modèle](#choosing-a-model).

### Budget

Plafonds journaliers et mensuels optionnels dans la devise de votre compte, plus une checklist **Seuils d’alerte** (par défaut 80% et 100%). Voir [Aperçu des budgets](#budgets-overview) et [Alertes de budget](#budget-alerts).

### Personnalité

L’**Invite initiale** est le prompt système qui définit le ton, le rôle et les règles de décision. Texte brut, pas de syntaxe de template. Voir [Personnalité et l’invite initiale](#personality-prompt).

### Contexte

Le fieldset Contexte contient trois cases à cocher, une zone de texte pour les directives, et les champs de portée :

- Inclure le commentaire parent et les réponses antérieures dans le même fil.
- Inclure le facteur de confiance du commentateur, l’ancienneté du compte, l’historique de bannissements et les commentaires récents.
- Inclure le titre de la page, le sous-titre, la description et les meta tags.
- Un bloc de texte optionnel **Directives communautaires** qui est préfixé à chaque prompt.
- **Restreindre à des pages spécifiques** - allowlist de motifs d’URL (une par ligne). Vide signifie pour l’ensemble du locataire.
- **Restreindre à des locales spécifiques** - allowlist de locales via un sélecteur à double liste. Vide signifie toutes les locales.

Plus de contexte permet de meilleures décisions mais augmente le coût en tokens par exécution. Voir [Options de contexte](#context-options), [Directives communautaires](#community-guidelines) et [Portée : filtres URL et locale](#scope-url-locale).

### Déclencheurs

Choisissez au moins un événement dans la liste. Pour les déclencheurs vote-threshold et flag-threshold vous devez aussi définir le seuil. Le champ optionnel **Délai avant exécution** reporte l’exécution après le déclenchement (utile pour les seuils de signalement où les votes sont encore en train de se stabiliser). Voir [Aperçu des événements de déclenchement](#triggers-overview) et [Déclencheurs différés](#trigger-deferred-delay).

### Appels d’outils autorisés

Cochez **Autoriser tous les appels d’outils** pour exposer la palette d’outils complète. Sinon, cochez les outils spécifiques que l’agent est autorisé à utiliser — les outils non autorisés sont retirés de la palette du modèle et refusés au moment du dispatch. La sous-section **Options de bannissement** verrouille les variantes destructrices de bannissement (delete-all-comments, ban-by-IP) derrière des opt-ins explicites. Voir [Aperçu des appels d’outils autorisés](#tools-overview) et [Outil : ban_user](#tool-ban-user).

### Approbations

Cochez les actions qui doivent être approuvées par un humain avant que l’agent ne les exécute. Les approbations ne s’appliquent qu’aux outils que l’agent est autorisé à invoquer ; les outils non autorisés sont refusés directement. Dans la région UE, **ban_user** est activé par défaut en vertu de l’article 17 du Digital Services Act. Voir [Flux d’approbation](#approval-workflow) et [Conformité UE DSA article 17](#eu-dsa-compliance).

### Notifications d’approbation

Si les approbations sont activées, choisissez qui reçoit des e-mails :

- **Tous les admins et modérateurs** - propriétaires de compte, super admins et admins modérateurs de commentaires.
- **Utilisateurs spécifiques** - sélection manuelle depuis un sélecteur à double liste.

La fréquence de livraison individuelle de chaque relecteur (immédiate, récapitulatif horaire, récapitulatif quotidien) est définie sur son propre profil. Voir [Notifications d’approbation](#approval-notifications).

### Statistiques

Lecture seule. Nombre total d’exécutions, horodatage de la dernière exécution, et l’ID du commentaire le plus récent écrit par l’agent (le cas échéant).

### Enregistrer

Cliquez sur **Enregistrer l’agent**. La page redirige vers la liste des agents. Les nouveaux agents sont immédiatement éligibles pour recevoir des déclencheurs en exécution à blanc.

### Modification ultérieure

Chaque ligne de la page de liste des agents expose des actions par agent : **Modifier**, **Dupliquer**, **Exécutions**, **Relectures**, **Exécution de test**, **Analyses**, **Approbations**, et **Supprimer**. Modifier un agent n’affecte pas rétroactivement les exécutions déjà enregistrées - l’historique est préservé. Les snapshots de replay figent aussi la configuration de l’agent au moment où le replay a été lancé, donc les résultats d’un replay sauvegardé restent reproductibles même après que vous ayez modifié le prompt.