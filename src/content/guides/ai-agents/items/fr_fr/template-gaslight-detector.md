---
**ID du modèle :** `gaslight_detector`

Le Gaslight Detector surveille les modifications de commentaires qui réécrivent l'historique au milieu d'une conversation — le type où un auteur change le sens d'un commentaire antérieur après que des réponses ont été écrites, laissant les réponses en aval hors contexte ou incorrectes. Quand l'agent estime qu'une modification franchit cette limite, il restaure le texte original et envoie un message direct (DM) à l'auteur pour expliquer.

C'est un modèle à risque plus élevé parce qu'il modifie le contenu utilisateur. Exécutez-le en [mode dry-run](#dry-run-mode) plus longtemps que vous ne le feriez pour un modèle en lecture seule, et placez `edit_comment` sous [approbation](#approval-workflow) jusqu'à ce que vous fassiez confiance au jugement du modèle sur votre trafic.

### Déclencheurs

- **Comment edited** (`COMMENT_EDIT`) - l'agent compare le texte nouveau et précédent et décide si la modification déforme des réponses déjà existantes.

Voir [Trigger: Comment Edited](#trigger-comment-edit) pour la charge utile complète, incluant le texte du commentaire précédent et le nombre de réponses au moment de la modification.

### Outils autorisés

- [`edit_comment`](#tool-edit-comment) - utilisé pour restaurer le texte original lorsque la modification est jugée être du gaslighting.
- [`warn_user`](#tool-warn-user) - émet un avertissement léger que l'utilisateur verra lors de sa prochaine visite.
- [`send_dm`](#tools-overview) - canal d'explication ; l'utilisateur reçoit un message direct décrivant pourquoi sa modification a été annulée.

Il ne peut pas bannir, marquer comme spam, voter, ou publier de nouveaux commentaires — la surface d'action est volontairement étroite.

### Ajouts recommandés avant mise en production

- **Placez `edit_comment` sous [approbation](#approval-workflow).** Rétablir un commentaire est visible par l'auteur et par quiconque a vu la version modifiée, donc un faux positif est embarrassant. Maintenez les approbations activées jusqu'à ce que le mode dry-run montre que l'agent est cohérent.
- **Affinez l'invite avec ce qui compte comme gaslighting sur votre site.** L'invite par défaut est courte volontairement. Donnez au modèle des exemples concrets — « inverser une affirmation oui/non », « supprimer un nombre cité par des réponses », « ajouter une phrase hostile après la publication des réponses » — et des non-exemples explicites comme corrections de fautes de frappe, nettoyage de formatage, ou ajout de sources.
- **Utilisez le nombre de réponses du contexte du déclencheur.** Les modifications de commentaires avec zéro réponse ne peuvent pas déformer une conversation ; l'invite doit dire au modèle d'ignorer ces cas.
- **Cochez « Include commenter's trust factor, account age, ban history, and recent comments »** dans les [Context Options](#context-options). Le modèle est beaucoup moins agressif lorsqu'il peut voir un compte de longue date agissant de bonne foi.
- **Envisagez une courte fenêtre de tolérance pour les modifications dans l'invite.** Beaucoup de modifications dans les 30 à 60 premières secondes sont des corrections de fautes ; demandez au modèle d'ignorer les modifications aussi rapides.

### Fenêtre d'exécution à sec recommandée

Faites fonctionner pendant au moins deux semaines de trafic réel en [mode dry-run](#dry-run-mode) avant de passer en Activé, et passez en revue chaque modification signalée pendant cette période. Utilisez [Test Runs (Replays)](#test-runs-replays) pour rejouer les 30 derniers jours de modifications contre l'agent avant la mise en production.

---