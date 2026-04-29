Quand l'agent met une approbation en file d'attente, la plateforme avertit les réviseurs par courriel. Deux paramètres sur le formulaire d'édition contrôlent ceci : **qui** est averti et **à quelle fréquence**.

### Who: notify mode

Deux modes :

- **All admins and moderators** (par défaut) - chaque propriétaire de compte, super admin et administrateur modérateur de commentaires du locataire est un réviseur potentiel.
- **Specific users** - sélectionnez manuellement une liste à partir d'un sélecteur à double liste sur le formulaire d'édition.

Dans les deux cas, un réviseur potentiel doit avoir un compte sur le locataire et une adresse courriel valide pour recevoir les notifications.

### How often: per-user frequency

Le **profil personnel** de chaque réviseur potentiel définit sa fréquence de notification pour les approbations d'agent :

- **Immediate** (par défaut) - un courriel par approbation en attente, envoyé dès que l'approbation est créée.
- **Hourly** - un courriel récapitulatif par heure résumant toutes les approbations mises en file durant cette heure.
- **Daily** - un courriel récapitulatif toutes les 24 heures.
- **Disabled** - aucun courriel. L'utilisateur peut toujours consulter et traiter les approbations via l'interface de la boîte de réception ; il n'est simplement pas averti par courriel.

L'utilisateur modifie ce paramètre dans son propre profil, pas sur le formulaire d'édition de l'agent. C'est volontaire : un locataire peut avoir dix agents, et un modérateur ne devrait pas avoir à définir sa fréquence préférée sur chaque agent individuellement.

### Cron jobs that drive digests

- **`hourly-agent-approval-digest`** - balaye chaque heure, regroupe les approbations mises en file depuis le dernier récapitulatif de chaque utilisateur, envoie un courriel par utilisateur.
- **`daily-agent-approval-digest`** - pareil, quotidiennement.
- **`agent-approval-reaper`** - supprime les approbations âgées de plus de 90 jours, quel que soit leur état.

Les tâches cron horaires et quotidiennes sont ciblées par destinataire : un utilisateur configuré sur la fréquence horaire est traité par la tâche horaire et ignoré par la quotidienne (et inversement). Les utilisateurs en fréquence immédiate sont notifiés par le chemin de code de création d'approbation, pas par les crons.

### Dedup state

La plateforme suit quels utilisateurs ont déjà reçu un courriel pour chaque approbation. Une fois qu'un utilisateur a été notifié (immédiatement ou dans un récapitulatif), il ne recevra plus de courriel pour la même approbation — même s'il change sa fréquence de immédiate à quotidienne en plein cycle.

### Approving from the email

Chaque courriel de notification contient un lien de connexion signé en un clic qui mène le réviseur directement à la page de détail de l'approbation, déjà authentifié. Il peut approuver, rejeter ou ouvrir le flux [Refine Prompts](#refining-prompts) à partir de là.

### What if no admins exist

Si `notifyMode` est `All admins and moderators` mais que le locataire n'a pas de super admins, d'administrateurs modérateurs de commentaires ou de propriétaires de compte avec des courriels valides, la plateforme enregistre un avertissement et l'approbation est quand même mise en file — simplement personne n'en est informé. Elle restera dans la boîte de réception jusqu'à ce que quelqu'un la consulte.

Si `notifyMode` est `Specific users` mais que vous n'avez sélectionné aucun utilisateur, même résultat.

### What if billing notifications are disabled

[Budget Alerts](#budget-alerts) — les courriels liés au budget — sont envoyés aux administrateurs de la facturation **indépendamment de la préférence de notification par utilisateur**. C'est volontaire : les dépassements de budget ont un impact sur les coûts, et le propriétaire de la facturation doit être informé.

Les notifications d'approbation respectent uniquement le paramètre de fréquence agent-approval par utilisateur. Elles ne tiennent pas compte de l'option générale de désabonnement aux notifications d'administration — un utilisateur qui s'est désabonné des notifications d'administration continuera de recevoir des courriels d'approbation s'il figure sur la liste des réviseurs, sauf si sa fréquence agent-approval est réglée sur **Disabled**.

### See also

- [Approval Workflow](#approval-workflow) pour le cycle de vie complet d'une approbation.
- [Refining Prompts](#refining-prompts) pour le flux « j'approuve sans cesse le même genre d'erreur ».