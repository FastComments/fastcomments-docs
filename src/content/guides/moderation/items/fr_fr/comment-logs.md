FastComments suit automatiquement des événements détaillés pour chaque commentaire afin de fournir de la transparence sur les décisions de modération et les actions du système. Ces journaux vous aident à comprendre pourquoi un commentaire a été approuvé, signalé comme spam ou a vu son statut modifié.

Vous pouvez consulter les journaux des commentaires pour des commentaires individuels dans le tableau de bord de modération des commentaires en sélectionnant un commentaire spécifique.

## Comment Log Events

Chaque commentaire conserve un journal des événements qui se produisent au cours de son cycle de vie. Ci-dessous les types d'événements qui sont suivis :

### Anonymization Events
- **Anonymized** - Le contenu du commentaire a été effacé et l'utilisateur marqué comme supprimé

### Approval Events
- **ApprovedDueToPastComment** - Commentaire approuvé parce que l'utilisateur a précédemment eu des commentaires approuvés
- **ApprovedIsAdmin** - Commentaire approuvé parce que l'utilisateur est un administrateur
- **NotApprovedRequiresApproval** - Le commentaire nécessite une approbation manuelle

### Spam Detection Events
- **IsSpam** - Commentaire signalé comme spam par le moteur de détection
- **IsSpamDueToBadWords** - Commentaire signalé comme spam en raison du filtre de grossièretés
- **IsSpamFromLLM** - Commentaire signalé comme spam par le moteur IA/LLM
- **IsSpamRepeatComment** - Commentaire signalé comme spam pour répétition
- **NotSpamIsOnlyImage** - Commentaire non signalé comme spam car il ne contient que des images
- **NotSpamIsOnlyReacts** - Commentaire non signalé comme spam car il ne contient que des réactions
- **NotSpamNoLinkOrMention** - Commentaire non signalé comme spam en raison de l'absence de liens ou mentions suspects
- **NotSpamPerfectTrustFactor** - Commentaire non signalé comme spam en raison d'un facteur de confiance élevé de l'utilisateur
- **NotSpamTooShort** - Commentaire non signalé comme spam car il est trop court pour être analysé
- **NotSpamSkipped** - La vérification anti-spam a été ignorée
- **NotSpamFromEngine** - Commentaire déterminé comme non-spam par le moteur de détection

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - La vérification du filtre de profanités a rencontré une erreur
- **BadWordsFoundBadPhrase** - Le filtre de profanités a détecté une phrase inappropriée
- **BadWordsFoundBadWord** - Le filtre de profanités a détecté un mot inapproprié
- **BadWordsNoDefinitionForLocale** - Aucune définition de profanités disponible pour la langue du commentaire

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Le commentaire nécessite une vérification mais l'utilisateur n'est pas dans une session vérifiée
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Le commentaire nécessite une vérification mais l'utilisateur n'est pas encore vérifié
- **InVerifiedSession** - L'utilisateur postant le commentaire est dans une session vérifiée
- **SentVerificationEmailNoSession** - E-mail de vérification envoyé à un utilisateur non vérifié
- **SentWelcomeEmail** - E-mail de bienvenue envoyé au nouvel utilisateur

### Trust and Security Events
- **TrustFactorChanged** - Le facteur de confiance de l'utilisateur a été modifié
- **SpamFilterDisabledBecauseAdmin** - Le filtrage anti-spam a été contourné pour un administrateur
- **TenantSpamFilterDisabled** - Le filtrage anti-spam a été désactivé pour l'ensemble du locataire
- **RepeatCommentCheckIgnored** - La vérification des commentaires répétés a été ignorée
- **UserIsAdmin** - Utilisateur identifié comme administrateur
- **UserIsAdminParentTenant** - Utilisateur identifié comme administrateur du locataire parent
- **UserIsAdminViaSSO** - Utilisateur identifié comme administrateur via SSO
- **UserIsMod** - Utilisateur identifié comme modérateur

### Comment Status Changes
- **ExpireStatusChanged** - Le statut d'expiration du commentaire a été modifié
- **ReviewStatusChanged** - Le statut d'examen du commentaire a été modifié
- **SpamStatusChanged** - Le statut de spam du commentaire a été mis à jour
- **ApproveStatusChanged** - Le statut d'approbation du commentaire a été modifié
- **TextChanged** - Le contenu textuel du commentaire a été modifié
- **VotesChanged** - Les comptes de votes du commentaire ont été mis à jour
- **Flagged** - Le commentaire a été signalé par des utilisateurs
- **UnFlagged** - Les signalements du commentaire ont été retirés

### Moderation Actions
- **Pinned** - Le commentaire a été épinglé par un modérateur
- **UnPinned** - Le commentaire a été désépinglé par un modérateur
- **RestoredFromAnonymized** - Le commentaire a été restauré depuis l'état anonymisé

### Notification Events
- **CreatedNotifications** - Des notifications ont été créées pour le commentaire
- **NotificationCreateFailure** - Échec de la création des notifications
- **BadgeAwarded** - Un badge a été attribué à l'utilisateur pour ce commentaire

### Publishing Events
- **PublishedLive** - Le commentaire a été publié aux abonnés en direct

### Integration Events
- **WebhookSynced** - Le commentaire a été synchronisé via webhook

### Spam Rule Events
- **SpamRuleMatch** - Le commentaire correspond à une règle anti-spam personnalisée

## Accessing Comment Logs

Les journaux des commentaires sont générés automatiquement et stockés avec chaque commentaire. Ils fournissent des informations précieuses pour :

- Comprendre les décisions de modération
- Déboguer les problèmes d'approbation/spam
- Suivre les comportements des utilisateurs
- Auditer les actions du système

Ces journaux aident à maintenir la transparence du processus de modération et assistent dans l'affinement du comportement de votre système de commentaires.