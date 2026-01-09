FastComments suit automatiquement des événements détaillés pour chaque commentaire afin de fournir de la transparence sur les décisions de modération et les actions du système. Ces journaux vous aident à comprendre pourquoi un commentaire a été approuvé, signalé comme spam ou a vu son statut modifié.

Vous pouvez consulter les journaux de commentaires pour chaque commentaire dans le tableau de bord Modérer les commentaires en sélectionnant un commentaire spécifique.

## Événements du journal des commentaires

Chaque commentaire conserve un journal des événements qui se produisent durant son cycle de vie. Voici les types d'événements qui sont suivis :

### Événements d'anonymisation
- **Anonymized** - Le contenu du commentaire a été effacé et l'utilisateur marqué comme supprimé

### Événements d'approbation
- **ApprovedDueToPastComment** - Commentaire approuvé parce que l'utilisateur a déjà des commentaires approuvés
- **ApprovedIsAdmin** - Commentaire approuvé parce que l'utilisateur est un administrateur
- **NotApprovedRequiresApproval** - Le commentaire nécessite une approbation manuelle

### Événements de détection de spam
- **IsSpam** - Commentaire signalé comme spam par le moteur de détection
- **IsSpamDueToBadWords** - Commentaire signalé comme spam en raison du filtre de propos inappropriés
- **IsSpamFromLLM** - Commentaire signalé comme spam par le moteur IA/LLM
- **IsSpamRepeatComment** - Commentaire signalé comme spam pour répétition
- **NotSpamIsOnlyImage** - Commentaire non signalé comme spam parce qu'il ne contient que des images
- **NotSpamIsOnlyReacts** - Commentaire non signalé comme spam parce qu'il ne contient que des réactions
- **NotSpamNoLinkOrMention** - Commentaire non signalé comme spam en raison de l'absence de liens ou de mentions suspectes
- **NotSpamPerfectTrustFactor** - Commentaire non signalé comme spam en raison d'un facteur de confiance utilisateur élevé
- **NotSpamTooShort** - Commentaire non signalé comme spam car trop court pour être analysé
- **NotSpamSkipped** - La vérification anti-spam a été ignorée
- **NotSpamFromEngine** - Commentaire déterminé comme non spam par le moteur de détection

### Événements liés aux propos inappropriés/grossièretés
- **BadWordsCheckFailed** - La vérification du filtre de propos inappropriés a rencontré une erreur
- **BadWordsFoundBadPhrase** - Le filtre de propos inappropriés a détecté une phrase inappropriée
- **BadWordsFoundBadWord** - Le filtre de propos inappropriés a détecté un mot inapproprié
- **BadWordsNoDefinitionForLocale** - Aucune définition de propos inappropriés disponible pour la langue du commentaire

### Événements de vérification de l'utilisateur
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Le commentaire nécessite une vérification mais l'utilisateur n'est pas dans une session vérifiée
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Le commentaire nécessite une vérification mais l'utilisateur n'est pas encore vérifié
- **InVerifiedSession** - L'utilisateur publiant le commentaire est dans une session vérifiée
- **SentVerificationEmailNoSession** - E-mail de vérification envoyé à un utilisateur non vérifié
- **SentWelcomeEmail** - E-mail de bienvenue envoyé au nouvel utilisateur

### Événements de confiance et de sécurité
- **TrustFactorChanged** - Le facteur de confiance de l'utilisateur a été modifié
- **SpamFilterDisabledBecauseAdmin** - Le filtrage anti-spam a été contourné pour un utilisateur administrateur
- **TenantSpamFilterDisabled** - Le filtrage anti-spam a été désactivé pour l'ensemble du locataire
- **RepeatCommentCheckIgnored** - La vérification des commentaires répétés a été ignorée
- **UserIsAdmin** - L'utilisateur a été identifié comme administrateur
- **UserIsAdminParentTenant** - L'utilisateur a été identifié comme administrateur du locataire parent
- **UserIsAdminViaSSO** - L'utilisateur a été identifié comme administrateur via SSO
- **UserIsMod** - L'utilisateur a été identifié comme modérateur

### Changements de statut du commentaire
- **ExpireStatusChanged** - Le statut d'expiration du commentaire a été modifié
- **ReviewStatusChanged** - Le statut de révision du commentaire a été modifié
- **SpamStatusChanged** - Le statut de spam du commentaire a été mis à jour
- **ApproveStatusChanged** - Le statut d'approbation du commentaire a été modifié
- **TextChanged** - Le contenu texte du commentaire a été modifié
- **VotesChanged** - Les comptes de votes du commentaire ont été mis à jour
- **Flagged** - Le commentaire a été signalé par des utilisateurs
- **UnFlagged** - Les signalements du commentaire ont été supprimés

### Actions de modération
- **Pinned** - Le commentaire a été épinglé par un modérateur
- **UnPinned** - Le commentaire a été désépinglé par un modérateur
- **RestoredFromAnonymized** - Le commentaire a été restauré depuis un état anonymisé

### Événements de notification
- **CreatedNotifications** - Des notifications ont été créées pour le commentaire
- **NotificationCreateFailure** - Échec de la création des notifications
- **BadgeAwarded** - Un badge utilisateur a été attribué pour le commentaire

### Événements de publication
- **PublishedLive** - Le commentaire a été publié aux abonnés en direct

### Événements d'intégration
- **WebhookSynced** - Le commentaire a été synchronisé via webhook

### Événements de règle anti-spam
- **SpamRuleMatch** - Le commentaire a correspondu à une règle anti-spam personnalisée

## Accéder aux journaux de commentaires

Les journaux de commentaires sont générés automatiquement et stockés avec chaque commentaire. Ils fournissent des informations précieuses pour :

- Comprendre les décisions de modération
- Déboguer les problèmes d'approbation/spam
- Suivre les comportements des utilisateurs
- Auditer les actions du système

Ces journaux contribuent à maintenir la transparence du processus de modération et aident à affiner le comportement de votre système de commentaires.