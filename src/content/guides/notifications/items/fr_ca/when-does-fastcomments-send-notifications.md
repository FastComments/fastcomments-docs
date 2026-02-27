---
Chez FastComments, nous savons que vous recevez déjà suffisamment de notifications. C'est pourquoi nous prenons des mesures pour limiter les notifications que reçoivent les utilisateurs tout en
les maintenant en contact avec leurs communautés. Nous voulons aussi tenir les administrateurs et modérateurs informés et leur indiquer quand une action est requise.

#### We'll send notifications for the following events for administrators and moderators:

- Résumé du digest de la communauté (fréquence configurable).
- Demandes d'aide et rappels de la communauté.
- Nouveaux commentaires.

#### For Commenters:

- Lorsqu'une personne répond à votre commentaire (par courriel).
- Lorsqu'on vous mentionne (notification dans l'application et par courriel).
- Lorsqu'une personne répond dans le même fil (notification dans l'application et par courriel).
- Lorsqu'une personne répond à un commentaire enfant dans le même fil (notification dans l'application et par courriel).
- Lorsqu'une personne répond sur une page à laquelle vous êtes abonné (notification dans l'application et par courriel, fréquence configurable par abonnement : chaque minute, toutes les heures ou quotidiennement).
- Lorsqu'un utilisateur commente pour la première fois (mais pas avec SSO).
- Lorsqu'un utilisateur laisse un commentaire durant une session non vérifiée (mais pas avec SSO).
  - Nous n'envoyons pas plusieurs courriels de vérification dans ce cas. Un seul est envoyé, le premier, qui vérifiera toute l'activité de la même session.

#### For All Users:

- Lorsqu'une connexion depuis une nouvelle adresse IP est détectée, un courriel d'alerte de sécurité est envoyé avec l'emplacement approximatif et l'adresse IP. Cela ne s'applique pas à la toute première connexion de l'utilisateur.

#### ...and finally for administrators only:

- Lorsque les intégrations sont terminées.
- Lorsque les migrations sont terminées.
- Lorsque les importations ou exportations sont terminées.
- En cas de problèmes de facturation.
- Rappels de fin d'essai.

Certaines notifications sont mises en lot afin d'éviter l'envoi massif de notifications aux utilisateurs. En savoir plus dans la section suivante `Notification Types`.

---