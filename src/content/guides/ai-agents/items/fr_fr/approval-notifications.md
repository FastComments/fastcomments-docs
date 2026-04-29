Quand l'agent met en file d'attente une approbation, la plateforme informe les réviseurs par e-mail. Deux paramètres du formulaire d'édition contrôlent cela : **qui** est informé et **à quelle fréquence**.

### Qui : mode de notification

Deux modes :

- **Tous les admins et modérateurs** (par défaut) - chaque propriétaire de compte, super administrateur et administrateur modérateur de commentaires du locataire est un réviseur potentiel.
- **Utilisateurs spécifiques** - sélectionnez manuellement une liste à partir d'un sélecteur à double liste sur le formulaire d'édition.

Dans les deux cas, un réviseur potentiel doit avoir un compte sur le locataire et une adresse e-mail valide pour recevoir les notifications.

### À quelle fréquence : fréquence par utilisateur

La **propre fiche** de chaque réviseur potentiel définit sa fréquence de notification personnelle pour les approbations d'agent :

- **Immédiat** (par défaut) - un e-mail par approbation en attente, envoyé dès que l'approbation est créée.
- **Toutes les heures** - un e-mail récapitulatif par heure résumant toutes les approbations mises en file d'attente durant cette heure.
- **Quotidien** - un e-mail récapitulatif toutes les 24 heures.
- **Désactivé** - aucun e-mail. L'utilisateur peut toujours examiner les approbations via l'interface de la boîte de réception ; il n'est simplement pas alerté.

L'utilisateur modifie ce paramètre sur sa propre fiche, pas sur le formulaire d'édition de l'agent. C'est volontaire : un locataire peut avoir dix agents, et un modérateur ne devrait pas avoir à définir sa fréquence préférée sur chaque agent indépendamment.

### Tâches cron qui alimentent les récapitulatifs

- **`hourly-agent-approval-digest`** - balaie toutes les heures, regroupe les approbations mises en file d'attente depuis le dernier récapitulatif de chaque utilisateur, envoie un e-mail par utilisateur.
- **`daily-agent-approval-digest`** - pareil, quotidiennement.
- **`agent-approval-reaper`** - supprime les approbations âgées de plus de 90 jours quel que soit leur état.

Les crons de récapitulatif horaire et quotidien sont ciblés par destinataire : un utilisateur avec la fréquence horaire est traité par le cron horaire et ignoré par le cron quotidien (et inversement). Les utilisateurs en fréquence immédiate sont notifiés par le chemin de création d'approbation, pas par les crons.

### État de déduplication

La plateforme suit les utilisateurs qui ont déjà reçu un e-mail pour chaque approbation. Une fois qu'un utilisateur a été notifié (immédiatement ou dans un récapitulatif), il ne recevra plus d'e-mail pour la même approbation — même s'il change sa fréquence d'immédiat à quotidien en cours de cycle.

### Approbation depuis l'e-mail

Chaque e-mail de notification contient un lien de connexion signé en un clic qui mène le réviseur directement à la page de détails de l'approbation, déjà authentifié. Il peut approuver, rejeter ou ouvrir le flux [Affiner les invites](#refining-prompts) à partir de là.

### Que se passe-t-il s'il n'y a pas d'administrateurs

Si `notifyMode` est `All admins and moderators` mais que le locataire n'a pas de super admins, d'administrateurs modérateurs de commentaires ou de propriétaires de compte avec des e-mails valides, la plateforme enregistre un avertissement et l'approbation est quand même mise en file d'attente — simplement personne n'en est informé. Elle restera dans la boîte de réception jusqu'à ce que quelqu'un la consulte.

Si `notifyMode` est `Specific users` mais que vous n'avez sélectionné aucun utilisateur, même résultat.

### Que se passe-t-il si les notifications de facturation sont désactivées

[Alertes de budget](#budget-alerts) - les e-mails liés au budget - sont envoyés aux administrateurs de facturation **indépendamment de la préférence de notification par utilisateur**. Cela est intentionnel : les dépassements de budget ont un impact sur les coûts, et le responsable de la facturation doit être informé.

Les notifications d'approbation respectent uniquement le paramètre de fréquence d'approbation d'agent par utilisateur. Elles ne tiennent pas compte de la désinscription plus large aux notifications d'administration - un utilisateur qui s'est désinscrit des notifications d'administration recevra néanmoins des e-mails d'approbation s'il figure dans la liste des réviseurs, sauf si sa fréquence d'approbation d'agent est réglée sur **Désactivé**.

### Voir aussi

- [Flux d'approbation](#approval-workflow) pour le cycle de vie complet d'une approbation.
- [Affiner les invites](#refining-prompts) pour le flux « J'approuve sans cesse le même type d'erreur ».