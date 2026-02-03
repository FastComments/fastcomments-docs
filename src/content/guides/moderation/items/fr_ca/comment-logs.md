FastComments enregistre automatiquement des événements détaillés pour chaque commentaire afin de fournir de la transparence sur les décisions de modération et les actions du système. Ces journaux vous aident à comprendre pourquoi un commentaire a été approuvé, signalé comme spam ou a vu son statut modifié.

## Accéder aux journaux de commentaires

Pour afficher les journaux d'un commentaire spécifique :

1. Accédez à la page **Modérer les commentaires** de votre tableau de bord FastComments
2. Trouvez le commentaire que vous souhaitez inspecter
3. Cliquez sur le bouton **Voir les journaux** (icône d'horloge) dans la barre d'actions du commentaire
4. Une boîte de dialogue s'ouvrira affichant l'historique complet des événements pour ce commentaire

Chaque entrée de journal affiche :
- **Quand** - L'horodatage de l'événement
- **Qui** - L'utilisateur ou le système qui a déclenché l'événement (le cas échéant)
- **Quoi** - Le type d'action ou d'événement
- **Détails** - Contexte supplémentaire tel que les valeurs avant/après, les noms des moteurs ou les données associées

## Événements du journal de commentaires

Chaque commentaire conserve un journal des événements survenus au cours de son cycle de vie. Voici les types d'événements qui sont enregistrés :

### Événements d'anonymisation
- **Anonymized** - Le contenu du commentaire a été effacé et l'utilisateur marqué comme supprimé
- **RestoredFromAnonymized** - Le commentaire a été restauré depuis l'état anonymisé

### Événements d'approbation
- **ApprovedDueToPastComment** - Commentaire approuvé parce que l'utilisateur a des commentaires précédemment approuvés (comprend une référence au commentaire précédent)
- **ApprovedIsAdmin** - Commentaire approuvé parce que l'utilisateur est administrateur
- **NotApprovedRequiresApproval** - Le commentaire nécessite une approbation manuelle
- **NotApprovedLowTrustFactor** - Commentaire non approuvé en raison d'un faible facteur de confiance de l'utilisateur (comprend la valeur du facteur de confiance)

### Événements d'approbation des commentaires de profil

Ces événements s'appliquent spécifiquement aux commentaires sur les profils d'utilisateurs :

- **ApprovedProfileAutoApproveAll** - Commentaire de profil approuvé automatiquement parce que le propriétaire du profil a activé l'approbation automatique pour tous les commentaires
- **ApprovedProfileTrusted** - Commentaire de profil approuvé parce que le commentateur est de confiance (comprend une référence au commentaire qui a établi la confiance)
- **NotApprovedProfileManualApproveAll** - Le commentaire de profil nécessite une approbation manuelle parce que le propriétaire du profil a activé l'approbation manuelle
- **NotApprovedProfileNotTrusted** - Commentaire de profil non approuvé parce que le commentateur n'est pas de confiance
- **NotApprovedProfileNewUser** - Commentaire de profil non approuvé parce que le commentateur est un nouvel utilisateur

### Événements de détection de spam
- **IsSpam** - Commentaire marqué comme spam par le moteur de détection (inclut quel moteur a pris la décision)
- **IsSpamDueToBadWords** - Commentaire marqué comme spam en raison du filtre de grossièretés
- **IsSpamFromLLM** - Commentaire marqué comme spam par un moteur AI/LLM (inclut le nom du moteur, la réponse et le nombre de jetons)
- **IsSpamRepeatComment** - Commentaire marqué comme spam pour répétition (inclut quel moteur l'a détecté)
- **NotSpamIsOnlyImage** - Commentaire non marqué comme spam parce qu'il ne contient que des images
- **NotSpamIsOnlyReacts** - Commentaire non marqué comme spam parce qu'il ne contient que des réactions
- **NotSpamNoLinkOrMention** - Commentaire non marqué comme spam en raison de l'absence de liens ou de mentions suspectes
- **NotSpamPerfectTrustFactor** - Commentaire non marqué comme spam en raison d'un facteur de confiance utilisateur élevé
- **NotSpamTooShort** - Commentaire non marqué comme spam parce qu'il est trop court pour être analysé
- **NotSpamSkipped** - La vérification anti-spam a été ignorée
- **NotSpamFromEngine** - Commentaire déterminé comme non spam par un moteur de détection (inclut le nom du moteur et le facteur de confiance)

### Événements de mots interdits/profanité
- **BadWordsCheckFailed** - La vérification du filtre de grossièretés a rencontré une erreur
- **BadWordsFoundBadPhrase** - Le filtre de grossièretés a détecté une phrase inappropriée (inclut la phrase)
- **BadWordsFoundBadWord** - Le filtre de grossièretés a détecté un mot inapproprié (inclut le mot)
- **BadWordsNoDefinitionForLocale** - Aucune définition de profanité disponible pour la langue du commentaire (inclut la locale)

### Événements de vérification de l'utilisateur
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Le commentaire nécessite une vérification mais l'utilisateur n'est pas dans une session vérifiée
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Le commentaire nécessite une vérification mais l'utilisateur n'est pas encore vérifié
- **InVerifiedSession** - L'utilisateur publiant le commentaire est dans une session vérifiée
- **SentVerificationEmailNoSession** - Courriel de vérification envoyé à un utilisateur non vérifié
- **SentWelcomeEmail** - Courriel de bienvenue envoyé au nouvel utilisateur

### Événements de confiance et de sécurité
- **TrustFactorChanged** - Le facteur de confiance de l'utilisateur a été modifié (inclut les valeurs avant et après)
- **SpamFilterDisabledBecauseAdmin** - Le filtrage du spam a été contourné pour un utilisateur administrateur
- **TenantSpamFilterDisabled** - Le filtrage du spam désactivé pour l'ensemble du locataire
- **RepeatCommentCheckIgnored** - La vérification des commentaires répétés a été contournée (inclut la raison)
- **UserIsAdmin** - L'utilisateur identifié comme administrateur
- **UserIsAdminParentTenant** - L'utilisateur identifié comme administrateur du locataire parent
- **UserIsAdminViaSSO** - L'utilisateur identifié comme administrateur via SSO
- **UserIsMod** - L'utilisateur identifié comme modérateur

### Modifications de statut du commentaire

Les événements de changement de statut incluent les valeurs avant et après, ainsi que l'utilisateur qui a effectué la modification :

- **ExpireStatusChanged** - Le statut d'expiration du commentaire a été modifié
- **ReviewStatusChanged** - Le statut de révision du commentaire a été changé
- **SpamStatusChanged** - Le statut de spam du commentaire a été mis à jour
- **ApproveStatusChanged** - Le statut d'approbation du commentaire a été modifié
- **TextChanged** - Le contenu textuel du commentaire a été édité (inclut le texte avant et après)
- **VotesChanged** - Les comptes de votes du commentaire ont été mis à jour (inclut une répartition détaillée des votes)
- **Flagged** - Le commentaire a été signalé par des utilisateurs
- **UnFlagged** - Les signalements du commentaire ont été supprimés

### Actions de modération
- **Pinned** - Le commentaire a été épinglé par un modérateur (inclut qui l'a épinglé)
- **UnPinned** - Le commentaire a été désépinglé par un modérateur (inclut qui l'a désépinglé)

### Événements de notification
- **CreatedNotifications** - Des notifications ont été créées pour le commentaire (inclut le nombre de notifications)
- **NotificationCreateFailure** - Échec de la création des notifications
- **BadgeAwarded** - Un badge utilisateur a été attribué pour le commentaire (inclut le nom du badge)

### Événements de publication
- **PublishedLive** - Le commentaire a été publié aux abonnés en direct (inclut le nombre d'abonnés)

### Événements d'intégration
- **WebhookSynced** - Le commentaire a été synchronisé via webhook

### Événements de règles anti-spam
- **SpamRuleMatch** - Le commentaire correspond à une règle anti-spam personnalisée (inclut les détails de la règle)

### Événements de localisation
- **LocaleDetectedFromText** - La locale linguistique a été détectée automatiquement à partir du texte du commentaire (inclut la langue et la locale détectées)

## Cas d'utilisation des journaux de commentaires

Les journaux de commentaires sont générés automatiquement et stockés avec chaque commentaire. Ils fournissent des informations précieuses pour :

- **Comprendre les décisions de modération** - Voir exactement pourquoi un commentaire a été approuvé, retenu pour examen ou marqué comme spam
- **Déboguer les problèmes d'approbation/anti-spam** - Retracer la logique de décision lorsque les commentaires ne se comportent pas comme prévu
- **Suivre les comportements des utilisateurs** - Surveiller les changements de facteur de confiance et le statut de vérification
- **Auditer les actions des modérateurs** - Examiner les actions effectuées par les modérateurs sur des commentaires spécifiques
- **Étudier l'efficacité du filtre anti-spam** - Voir quels moteurs de détection attrapent le spam et lesquels ne le font pas
- **Dépanner les intégrations** - Vérifier les synchronisations de webhook et la livraison des notifications

Ces journaux contribuent à maintenir la transparence du processus de modération et aident à affiner le comportement de votre système de commentaires.