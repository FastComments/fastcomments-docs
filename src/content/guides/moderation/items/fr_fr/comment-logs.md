FastComments suit automatiquement le suivi d'événements détaillés pour chaque commentaire afin de fournir de la transparence sur les décisions de modération et les actions du système. Ces journaux vous aident à comprendre pourquoi un commentaire a été approuvé, signalé comme spam ou dont le statut a été modifié.

## Accéder aux journaux de commentaires

Pour afficher les journaux d'un commentaire spécifique :

1. Accédez à la page **Modérer les commentaires** de votre tableau de bord FastComments  
2. Trouvez le commentaire que vous souhaitez inspecter  
3. Cliquez sur le bouton **Afficher les journaux** (icône d'horloge) dans la barre d'actions du commentaire  
4. Une boîte de dialogue apparaîtra affichant l'historique complet des événements pour ce commentaire  

Chaque entrée du journal affiche :
- **Quand** – L'horodatage de l'événement  
- **Qui** – L'utilisateur ou le système qui a déclenché l'événement (le cas échéant)  
- **Quoi** – Le type d'action ou d'événement  
- **Détails** – Contexte supplémentaire tel que les valeurs avant/après, les noms de moteur ou les données associées  

## Événements du journal de commentaires

Chaque commentaire conserve un journal des événements qui se produisent au cours de son cycle de vie. Voici les types d'événements qui sont suivis :

### Événements d'anonymisation
- **Anonymisé** – Le contenu du commentaire a été effacé et l'utilisateur marqué comme supprimé  
- **RestauréDepuisAnonymisé** – Le commentaire a été restauré depuis l'état anonymisé  

### Événements d'approbation
- **ApprouvéEnRaisonDUnCommentaireAntérieur** – Commentaire approuvé parce que l'utilisateur a déjà approuvé des commentaires auparavant (inclut la référence au commentaire antérieur)  
- **ApprouvéEstAdmin** – Commentaire approuvé parce que l'utilisateur est un administrateur  
- **NonApprouvéNécessiteApprobation** – Le commentaire nécessite une approbation manuelle  
- **NonApprouvéFaibleFacteurDeConfiance** – Commentaire non approuvé en raison d'un faible facteur de confiance de l'utilisateur (inclut la valeur du facteur de confiance)  

### Événements d'approbation de commentaires de profil
Ces événements s'appliquent spécifiquement aux commentaires sur les profils d'utilisateurs :

- **ProfilCommentaireAutoApprouvéTous** – Commentaire de profil auto-approuvé parce que le propriétaire du profil a activé l'auto-approbation pour tous les commentaires  
- **ProfilCommentaireApprouvéConfié** – Commentaire de profil approuvé parce que le commentateur est de confiance (inclut la référence au commentaire qui a établi la confiance)  
- **ProfilCommentaireNonApprouvéApprobationManuelleTous** – Commentaire de profil nécessite une approbation manuelle parce que le propriétaire du profil a activé l'approbation manuelle  
- **ProfilCommentaireNonApprouvéNonConfié** – Commentaire de profil non approuvé parce que le commentateur n'est pas de confiance  
- **ProfilCommentaireNonApprouvéNouvelUtilisateur** – Commentaire de profil non approuvé parce que le commentateur est un nouvel utilisateur  

### Événements de détection de spam
- **EstSpam** – Commentaire signalé comme spam par le moteur de détection (inclut le moteur qui a pris la décision)  
- **EstSpamEnRaisonDeMotsInappropriés** – Commentaire signalé comme spam en raison du filtre de profanity  
- **EstSpamDepuisLLM** – Commentaire signalé comme spam par le moteur IA/LLM (inclut le nom du moteur, la réponse et le nombre de tokens)  
- **EstSpamCommentaireRépetitif** – Commentaire signalé comme spam pour être répétitif (inclut le moteur qui l'a détecté)  
- **NonSpamEstSeulementImage** – Commentaire non signalé comme spam car il ne contient que des images  
- **NonSpamEstSeulementRéactions** – Commentaire non signalé comme spam car il ne contient que des réactions  
- **NonSpamPasDeLienOuMention** – Commentaire non signalé comme spam car il n'y a pas de liens ou de mentions suspectes  
- **NonSpamFacteurDeConfianceParfait** – Commentaire non signalé comme spam en raison d'une confiance élevée de l'utilisateur  
- **NonSpamTropCourt** – Commentaire non signalé comme spam car il est trop court pour être analysé  
- **NonSpamIgnoré** – La vérification du spam a été ignorée  
- **NonSpamDepuisMoteur** – Commentaire déterminé comme non spam par le moteur de détection (inclut le nom du moteur et le facteur de confiance)  

### Événements de mots inappropriés/profanité
- **ÉchecVérificationMotsInappropriés** – La vérification du filtre de profanity a rencontré une erreur  
- **MotsInappropriésPhraseDétectée** – Le filtre de profanity a détecté une phrase inappropriée (inclut la phrase)  
- **MotsInappropriésMotDétecté** – Le filtre de profanity a détecté un mot inapproprié (inclut le mot)  
- **MotsInappropriésPasDeDéfinitionPourLocale** – Aucune définition de profanity disponible pour la langue du commentaire (inclut la locale)  

### Événements de vérification d'utilisateur
- **CommentaireDoitÊtreVérifiéPourApprouverPasDansSessionVérifiée** – Le commentaire nécessite une vérification mais l'utilisateur n'est pas dans une session vérifiée  
- **CommentaireDoitÊtreVérifiéPourApprouverPasEncoreVérifié** – Le commentaire nécessite une vérification mais l'utilisateur n'est pas encore vérifié  
- **DansSessionVérifiée** – L'utilisateur qui publie le commentaire est dans une session vérifiée  
- **EmailVérificationEnvoyéPasDeSession** – Email de vérification envoyé à un utilisateur non vérifié  
- **EmailBienvenueEnvoyé** – Email de bienvenue envoyé au nouvel utilisateur  

### Événements de confiance et de sécurité
- **FacteurDeConfianceModifié** – Le facteur de confiance de l'utilisateur a été modifié (inclut les valeurs avant et après)  
- **FiltreSpamDésactivéParceQuAdmin** – Filtrage du spam contourné pour l'utilisateur admin  
- **FiltreSpamLocataireDésactivé** – Filtrage du spam désactivé pour l'ensemble du locataire  
- **VérificationCommentaireRépetitifIgnorée** – La vérification des commentaires répétitifs a été contournée (inclut la raison)  
- **UtilisateurEstAdmin** – Utilisateur identifié comme admin  
- **UtilisateurEstAdminLocataireParent** – Utilisateur identifié comme admin du locataire parent  
- **UtilisateurEstAdminViaSSO** – Utilisateur identifié comme admin via SSO  
- **UtilisateurEstModérateur** – Utilisateur identifié comme modérateur  

### Modifications du statut du commentaire
Les événements de changement de statut incluent les valeurs avant et après, ainsi que l'utilisateur qui a effectué le changement :

- **StatutExpirationModifié** – Le statut d'expiration du commentaire a été modifié  
- **StatutRévisionModifié** – Le statut de révision du commentaire a été changé  
- **StatutSpamModifié** – Le statut de spam du commentaire a été mis à jour  
- **StatutApprobationModifié** – Le statut d'approbation du commentaire a été changé  
- **TexteModifié** – Le contenu texte du commentaire a été édité (inclut le texte avant et après)  
- **VotesModifiés** – Le nombre de votes du commentaire a été mis à jour (inclut la répartition détaillée des votes)  
- **Signalé** – Le commentaire a été signalé par les utilisateurs  
- **NonSignalé** – Les signalements du commentaire ont été retirés  

### Actions de modération
- **Épinglé** – Le commentaire a été épinglé par le modérateur (inclut qui l'a épinglé)  
- **Désépinglé** – Le commentaire a été désépinglé par le modérateur (inclut qui l'a désépinglé)  

### Événements de notification
- **NotificationsCréées** – Des notifications ont été créées pour le commentaire (inclut le nombre de notifications)  
- **ÉchecCréationNotification** – Échec de la création des notifications  
- **BadgeAttribué** – Un badge utilisateur a été attribué pour le commentaire (inclut le nom du badge)  

### Événements de mention et de notification de réponse
Ces événements indiquent la personne qui recevrait l'e‑mail ou la notification. Lorsqu'aucun envoi n'a eu lieu, la colonne Détails indique la raison.

- **EmailMentionEnvoyé** – Un utilisateur mentionné dans le commentaire a reçu un e‑mail  
- **EmailMentionIgnoré** – Un utilisateur mentionné n'a pas reçu d'e‑mail (inclut la raison)  
- **MentionEnAttenteDApprobation** – L'e‑mail de mention attend que le commentaire soit approuvé  
- **NotificationMentionCréée** – Un utilisateur mentionné a reçu une notification dans l'application  
- **NotificationMentionIgnorée** – Un utilisateur mentionné n'a pas reçu de notification dans l'application (inclut la raison)  
- **EmailRéponseEnvoyé** – L'auteur du commentaire auquel on répond a reçu un e‑mail à propos de cette réponse  
- **EmailRéponseIgnoré** – L'auteur du commentaire auquel on répond n'a pas reçu d'e‑mail (inclut la raison)  
- **NotificationRéponseIgnorée** – L'auteur du commentaire auquel on répond n'a pas reçu de notification dans l'application (inclut la raison)  

Raisons affichées lorsqu'un e‑mail ou une notification n'a pas été envoyé :

- L'utilisateur n'existe plus, ou n'a pas d'adresse e‑mail  
- L'utilisateur a désactivé les notifications par e‑mail, ou désactivé les notifications pour ce fil  
- L'un des deux utilisateurs a bloqué l'autre  
- Les utilisateurs ne sont dans aucun des mêmes groupes SSO  
- L'adresse e‑mail de l'utilisateur est sur la liste de suppression après un rebond ou une plainte pour spam (voir [Gestion de la suppression des e‑mails](/guide-notifications.html#email-suppression-management))  
- L'adresse e‑mail de l'utilisateur est chez example.com, qui ne peut pas recevoir d'e‑mail  
- Le commentaire a été marqué comme spam, supprimé, ou non approuvé dans les 7 jours  
- Le commentaire auquel on répond a été laissé anonymement  
- L'utilisateur a répondu à son propre commentaire  
- L'utilisateur a été mentionné dans la réponse, il a donc reçu l'e‑mail de mention au lieu de l'e‑mail de réponse  
- L'utilisateur avait déjà une notification de réponse pour le commentaire  
- L'envoi a échoué 5 fois  

Si la livraison échoue ou atteint une limite d'envoi, l'e‑mail est mis en file d'attente pour réessayer et l'entrée du journal l'indique.

### Événnements de publication
- **PubliéEnDirect** – Le commentaire a été publié aux abonnés en direct (inclut le nombre d'abonnés)  

### Événements d'intégration
- **WebhookSynchronisé** – Le commentaire a été synchronisé via webhook  

### Événements de règle de spam
- **CorrespondanceRègleSpam** – Le commentaire a correspondu à une règle de spam personnalisée (inclut les détails de la règle)  

### Événements de localisation
- **LocaleDétectéDepuisTexte** – La locale de langue a été détectée automatiquement à partir du texte du commentaire (inclut la langue et la locale détectées)  

## Cas d'utilisation des journaux de commentaires

Les journaux de commentaires sont générés automatiquement et stockés avec chaque commentaire. Ils offrent des informations précieuses pour :

- **Comprendre les décisions de modération** – Voir exactement pourquoi un commentaire a été approuvé, mis en attente de révision ou marqué comme spam  
- **Déboguer les problèmes d'approbation/spam** – Suivre la logique de décision lorsque les commentaires ne se comportent pas comme prévu  
- **Suivre les modèles de comportement des utilisateurs** – Surveiller les changements de facteur de confiance et le statut de vérification  
- **Auditer les actions des modérateurs** – Examiner les actions que les modérateurs ont prises sur des commentaires spécifiques  
- **Investiguer l'efficacité du filtre anti‑spam** – Voir quels moteurs de détection attrapent le spam et lesquels ne le font pas  
- **Déboguer les intégrations** – Vérifier les synchronisations webhook et la livraison des notifications  

Ces journaux aident à maintenir la transparence du processus de modération et à affiner le comportement de votre système de commentaires.