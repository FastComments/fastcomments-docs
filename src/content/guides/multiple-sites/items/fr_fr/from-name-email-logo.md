Parfois, FastComments doit envoyer des e-mails à vos utilisateurs, surtout si vous n'utilisez pas Secure SSO.

Des exemples incluent la vérification de leur compte ou de leur activité lorsqu'ils commentent pour la première fois. FastComments
leur enverra également des notifications pour les réponses à leurs commentaires.

Lorsque FastComments envoie des e-mails à vos utilisateurs, nous utiliserons un nom et une adresse e-mail d'expéditeur par défaut : `FastComments Robot` et `noreply@fastcomments.com`.

Nous utiliserons également notre propre logo dans le pied de page de ces e-mails.

Si vous utilisez FastComments Flex ou Pro, tout cela peut être personnalisé par domaine via la "page Mes domaines" :

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Lorsque vous personnalisez le logo affiché dans les e-mails, assurez-vous que la taille que vous téléchargez correspond à la taille que vous souhaitez afficher dans le pied de page de l'e-mail.

### Lors de la personnalisation du `From Domain`

Si vous personnalisez le `From Domain`, les fournisseurs et clients de messagerie doivent savoir que FastComments est autorisé à envoyer des e-mails en votre nom. Sinon,
définir le `From Domain` sans suivre les étapes ci-dessous entraînera probablement l'envoi des e-mails dans les spams.

#### 1. Configurer SPF

Pour permettre à FastComments d'envoyer des e-mails de façon sécurisée au nom de votre domaine, assurez-vous d'ajouter un enregistrement SPF qui nous y autorise.

Assurez-vous qu'il existe des enregistrements SPF autorisant `mail.fastcomments.com` et `sib.fastcomments.com` à envoyer des e-mails au nom de votre domaine.

Plus d'informations sur la façon de procéder sont disponibles ici : https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configurer DKIM

En plus du SPF, vous devez configurer DKIM. Une fois votre configuration DNS prête, vous pouvez cliquer sur "Afficher Avancé" sur la page de configuration du domaine
pour afficher les paramètres DKIM par domaine.

Vous pouvez également [appeler l'API](/guide-api.html#domain-config-structure) pour configurer DKIM.

### Liens de désinscription

Lors de l'utilisation du SSO, les fonctionnalités de désinscription utilisées dans les e-mails et les notifications peuvent être personnalisées [via l'API DomainConfigs](/guide-api.html#domain-config-structure).