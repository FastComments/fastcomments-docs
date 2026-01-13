Parfois, FastComments doit envoyer des courriels à vos utilisateurs, surtout si vous n'utilisez pas Secure SSO.

Des exemples incluent la vérification de leur compte ou de leur activité lors de leur premier commentaire. FastComments leur enverra aussi des notifications pour les réponses à leurs commentaires.

Lorsque FastComments envoie des courriels à vos utilisateurs, nous utiliserons un Nom et une Adresse de l'expéditeur par défaut de `FastComments Robot` et `noreply@fastcomments.com`.

Nous utiliserons également notre propre logo dans le pied de page de ces courriels.

Si vous avez FastComments Flex ou Pro, tout cela peut être personnalisé par domaine via la page « Mes domaines » :

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Lorsque vous personnalisez le logo affiché dans les courriels, assurez-vous que la taille que vous téléversez est la même que celle que vous souhaitez afficher dans le pied de page du courriel.

### Lors de la personnalisation du `From Domain`

Si vous personnalisez le `From Domain`, les fournisseurs et clients de messagerie doivent savoir que FastComments est autorisé à envoyer des courriels en votre nom. Sinon,
définir le `From Domain` sans suivre les étapes ci-dessous entraînera probablement l'envoi des courriels dans le dossier de courrier indésirable.

#### 1. Configurer le SPF

Pour permettre à FastComments d'envoyer en toute sécurité des courriels en tant que votre domaine, assurez-vous d'ajouter un enregistrement SPF qui nous autorise à le faire.

Assurez-vous qu'il existe des enregistrements SPF permettant à `mail.fastcomments.com` et `sib.fastcomments.com` d'envoyer des courriels en tant que votre domaine.

Plus d'informations sur la manière de procéder ici : https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configurer le DKIM

En plus du SPF, vous devriez configurer le DKIM. Une fois votre configuration DNS prête, vous pouvez cliquer sur « Show Advanced » dans la page de configuration de domaine
pour afficher les paramètres DKIM par domaine.

Vous pouvez aussi [invoke the API](/guide-api.html#domain-config-structure) pour définir la configuration DKIM.

### Liens de désabonnement

Lorsque vous utilisez SSO, les fonctionnalités de désabonnement utilisées dans les courriels et les notifications peuvent être personnalisées [via the DomainConfigs API](/guide-api.html#domain-config-structure).