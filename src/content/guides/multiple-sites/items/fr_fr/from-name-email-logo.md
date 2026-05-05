Parfois FastComments doit envoyer des e-mails à vos utilisateurs, en particulier si vous n'utilisez pas Secure SSO.

Des exemples incluent la vérification de leur compte ou de leur activité lors du premier commentaire. FastComments leur enverra également des notifications pour les réponses à leurs commentaires.

Lorsque FastComments envoie des e-mails à vos utilisateurs, nous utiliserons par défaut comme nom et adresse d'expéditeur `FastComments Robot` et `noreply@fastcomments.com`.

Nous utiliserons également notre propre logo dans le pied de page de ces e-mails.

Si vous avez FastComments Flex ou Pro, tout cela peut être personnalisé par domaine via la page "Mes domaines" :

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Lorsque vous personnalisez le logo affiché dans les e-mails, assurez-vous que la taille que vous téléversez correspond à la taille que vous souhaitez afficher dans le pied de page de l'e-mail.

### Lors de la personnalisation du `From Domain`

Si vous personnalisez le `From Domain`, les fournisseurs et clients de messagerie doivent savoir que FastComments est autorisé à envoyer des e-mails en votre nom. Sinon, définir le `From Domain` sans suivre les étapes ci-dessous entraînera probablement l'envoi des e-mails dans les spams.

#### 1. Configurer SPF

Pour permettre à FastComments d'envoyer en toute sécurité des e-mails au nom de votre domaine, assurez-vous d'ajouter un enregistrement SPF qui nous y autorise.

Assurez-vous qu'il existe des enregistrements SPF permettant à `mail.fastcomments.com` et `sib.fastcomments.com` d'envoyer des e-mails au nom de votre domaine.

Plus d'informations sur la manière de procéder sont disponibles ici : https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configurer DKIM

En plus du SPF, vous devez configurer DKIM. Une fois votre configuration DNS prête, vous pouvez cliquer sur "Afficher les options avancées" dans la page de configuration des domaines pour afficher les paramètres DKIM par domaine.

Vous pouvez également [invoquer l'API](/guide-api.html#domain-config-structure) pour définir la configuration DKIM.

### Liens de désabonnement

Lors de l'utilisation de SSO, les fonctionnalités de désabonnement utilisées dans les e-mails et les notifications peuvent être personnalisées via la DomainConfigs API[/guide-api.html#domain-config-structure].

### Obfuscation des liens des e-mails

Si la réputation du domaine de votre site fait que les e-mails de notification arrivent dans les spams, vous pouvez faire passer les boutons "view comment" par `fastcomments.com` au lieu de lier directement vers votre page. Les fournisseurs de boîtes mail évaluent chaque lien dans le corps de l'e-mail en fonction de la réputation de la destination, donc si votre domaine est signalé, les liens bruts contribuent au score de spam indépendamment de la qualité de votre configuration d'envoi.

Activez ceci sous "Afficher les options avancées" sur la page Mes domaines, dans la section "Obfuscation des liens des e-mails". Ce paramètre est par domaine.

Lorsqu'elle est activée, les liens dans les e-mails mention, reply, new-comment, subscribed-page, profile-comment et digest sont réécrits en jetons courts qui redirigent vers la page d'origine lors du clic. La destination est liée à votre locataire : la redirection ne transmet que vers des URL dont l'hôte correspond à l'un de vos domaines configurés, et les jetons expirent automatiquement après 30 jours.

L'expérience lors du clic reste inchangée. Les lecteurs arrivent toujours sur votre page avec le commentaire affiché à l'écran.