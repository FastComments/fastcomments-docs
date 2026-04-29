Depuis la [page des Agents IA](https://fastcomments.com/auth/my-account/ai-agents) vous pouvez créer un agent de deux façons :

- **À partir d’un modèle.** Cliquez sur **Parcourir les modèles** et choisissez l’un des quatre agents de démarrage intégrés. Le formulaire arrive prérempli et le statut de l’agent est **Exécution simulée**. Voir [Starter Templates](#starter-templates).
- **À partir de zéro.** Cliquez sur **Créer un nouvel agent**. Le formulaire arrive vide.

Dans les deux cas, c’est le même formulaire d’édition que vous enregistrerez et modifierez par la suite. Cette page parcourt le formulaire de haut en bas.

### Principes de base

- **Nom interne.** Un identifiant court utilisé uniquement dans les tableaux de bord d’administration (historique des exécutions, analyses, journaux d’audit). Les minuscules avec underscores fonctionnent bien : `moderator`, `welcome_greeter`. Si le nom interne d’un modèle est déjà pris, le formulaire ajoute automatiquement un suffixe (`tos_enforcer_2`, etc.).
- **Nom affiché.** Affiché publiquement chaque fois que l’agent publie un commentaire. C’est ce que voient vos lecteurs.
- **Statut.** Désactivé, Exécution simulée, ou Activé. Les nouveaux agents sont toujours par défaut en Exécution simulée. Voir [Status States](#status-states).

### Modèle

Choisissez le LLM. Voir [Choosing a Model](#choosing-a-model).

### Budget

Plafonds quotidiens et mensuels optionnels dans la devise de votre compte, plus une liste de contrôle **Seuils d’alerte** (par défaut 80 % et 100 %). Voir [Budgets Overview](#budgets-overview) et [Budget Alerts](#budget-alerts).

### Personnalité

L’**Invite initiale** est le prompt système qui définit le ton, le rôle et les règles de décision. Texte brut, pas de syntaxe de modèle. Voir [Personality and the Initial Prompt](#personality-prompt).

### Contexte

Le groupe de champs Contexte contient trois cases à cocher, une zone de texte pour des directives, et les entrées de périmètre :

- Inclure le commentaire parent et les réponses précédentes dans le même fil.
- Inclure le facteur de confiance du commentateur, l’ancienneté du compte, l’historique des bannissements et les commentaires récents.
- Inclure le titre de la page, le sous-titre, la description et les balises meta.
- Un bloc de texte optionnel **Directives communautaires** qui est préfixé à chaque prompt.
- **Restreindre à des pages spécifiques** - allowlist de motifs d’URL (un par ligne). Vide signifie à l’échelle du locataire.
- **Restreindre à des locales spécifiques** - allowlist de locales via un sélecteur à double liste. Vide signifie toutes les locales.

Plus de contexte produit de meilleures décisions mais augmente le coût en tokens par exécution. Voir [Context Options](#context-options), [Community Guidelines](#community-guidelines), et [Scope: URL and Locale Filters](#scope-url-locale).

### Déclencheurs

Choisissez au moins un événement dans la liste. Pour les déclencheurs vote-threshold et flag-threshold vous devez également définir le seuil. Le champ optionnel **Délai avant exécution** diffère l’exécution après qu’un déclencheur se produise (utile pour les seuils de signalement où les votes sont encore en cours de stabilisation). Voir [Trigger Events Overview](#triggers-overview) et [Deferred Triggers](#trigger-deferred-delay).

### Appels d’outils autorisés

Cochez **Autoriser tous les appels d’outils** pour exposer la palette complète d’outils. Sinon, cochez les outils spécifiques que l’agent est autorisé à utiliser - les outils non autorisés sont retirés de la palette du modèle et refusés au moment de l’envoi. La sous-section **Options de bannissement** restreint les variantes destructrices de bannissement (delete-all-comments, ban-by-IP) derrière des autorisations explicites. Voir [Allowed Tool Calls Overview](#tools-overview) et [Tool: ban_user](#tool-ban-user).

### Approbations

Cochez les actions qui doivent être approuvées par un humain avant que l’agent ne les exécute. Les approbations ne s’appliquent qu’aux outils que l’agent est autorisé à invoquer ; les outils non autorisés sont refusés d’emblée. Dans la région UE, ban_user est activé conformément à l’article 17 du Digital Services Act. Voir [Approval Workflow](#approval-workflow) et [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Notifications d’approbation

Si les approbations sont activées, choisissez qui reçoit un courriel :

- **Tous les administrateurs et modérateurs** - propriétaires de compte, super administrateurs, et administrateurs modérateurs de commentaires.
- **Utilisateurs spécifiques** - choisis manuellement à partir d’un sélecteur à double liste.

La fréquence d’envoi individuelle de chaque réviseur (immédiat, résumé horaire, résumé quotidien) est définie dans son propre profil. Voir [Approval Notifications](#approval-notifications).

### Statistiques

Lecture seule. Nombre total d’exécutions, horodatage de la dernière exécution, et l’ID du commentaire le plus récent écrit par l’agent (le cas échéant).

### Enregistrer

Cliquez sur **Enregistrer l’agent**. La page redirige vers la liste des agents. Les nouveaux agents sont immédiatement éligibles pour recevoir des déclencheurs en exécution simulée.

### Édition ultérieure

Chaque ligne de la page de liste des agents expose des actions par agent : **Modifier**, **Dupliquer**, **Exécutions**, **Replays**, **Exécution de test**, **Analyses**, **Approbations**, et **Supprimer**. Modifier un agent n’affecte pas rétroactivement les exécutions déjà enregistrées - l’historique est préservé. Les instantanés de replay figent aussi la configuration de l’agent au moment où le replay a été lancé, de sorte que les résultats d’un replay enregistré restent reproductibles même après que vous ayez modifié le prompt.