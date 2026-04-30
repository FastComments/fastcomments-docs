**ID du modèle :** `gaslight_detector`

Le Gaslight Detector surveille les modifications de commentaires qui réécrivent l'histoire au milieu d'une conversation — le genre où un auteur change le sens d'un commentaire antérieur après que des réponses ont été écrites, laissant les réponses en aval hors de contexte ou incorrectes. Lorsque l'agent juge qu'une modification franchit cette ligne, il restaure le texte original et envoie un message direct (DM) à l'auteur pour expliquer.

Ceci est un modèle à risque élevé parce qu'il modifie le contenu des utilisateurs. Exécutez-le en [dry-run](#dry-run-mode) plus longtemps que vous ne le feriez pour un modèle en lecture seule, et placez `edit_comment` derrière un [approval](#approval-workflow) jusqu'à ce que vous ayez confiance dans le jugement du modèle sur votre trafic.

### Déclencheurs

- **Comment edited** (`COMMENT_EDIT`) - l'agent compare le nouveau texte et le texte précédent et décide si la modification déforme des réponses déjà existantes.

Voir [Déclencheur : Commentaire modifié](#trigger-comment-edit) pour la charge utile complète, incluant le texte du commentaire précédent et le nombre de réponses au moment de la modification.

### Outils autorisés

- [`edit_comment`](#tool-edit-comment) - utilisé pour restaurer le texte original lorsque la modification est jugée comme du gaslighting.
- [`warn_user`](#tool-warn-user) - émet un avertissement léger que l'utilisateur verra à sa prochaine visite.
- [`send_dm`](#tools-overview) - le canal d'explication ; l'utilisateur reçoit un message direct décrivant pourquoi sa modification a été annulée.

Il ne peut pas bannir, marquer comme spam, voter ou publier de nouveaux commentaires — la surface est volontairement restreinte.

### Ajouts recommandés avant la mise en production

- **Placez `edit_comment` derrière un [approval](#approval-workflow).** Rétablir un commentaire est visible par l'auteur et par toute personne ayant vu la version modifiée, donc un faux positif est embarrassant. Maintenez les approbations activées jusqu'à ce que le dry-run montre que l'agent est cohérent.
- **Affinez l'invite avec ce qui compte comme gaslighting sur votre site.** L'invite par défaut est volontairement courte. Donnez au modèle des exemples concrets — "inverser une affirmation oui/non", "supprimer un chiffre que les réponses citent", "ajouter une phrase hostile après la publication de réponses" — et des non-exemples explicites comme les corrections de fautes de frappe, le nettoyage de la mise en forme ou l'ajout de sources.
- **Utilisez le nombre de réponses du contexte du déclencheur.** Les modifications apportées à des commentaires sans réponses ne peuvent pas déformer une conversation ; l'invite devrait demander au modèle d'ignorer ces cas.
- **Cochez "Inclure le facteur de confiance du commentateur, l'ancienneté du compte, l'historique de bannissement et les commentaires récents"** dans les [Context Options](#context-options). Le modèle est beaucoup moins agressif lorsqu'il peut voir un compte ancien et de bonne foi.
- **Envisagez une courte période de grâce pour les modifications dans l'invite.** De nombreuses modifications effectuées dans les 30 à 60 premières secondes sont des corrections de fautes de frappe ; demandez au modèle d'ignorer les modifications réalisées si rapidement.

### Fenêtre recommandée en mode [dry-run]

Exécutez pendant au moins deux semaines de trafic réel en [dry-run](#dry-run-mode) avant de passer en Enabled, et examinez chaque modification signalée pendant cette période. Utilisez les [Test Runs (Replays)](#test-runs-replays) pour rejouer les 30 derniers jours de modifications contre l'agent avant la mise en production.