Parfois, FastComments doit envoyer des courriels à vos utilisateurs, surtout si vous n'utilisez pas Secure SSO.

Parmi les exemples, on compte la vérification de leur compte ou de leur activité lorsqu'ils commentent pour la première fois. FastComments
leur enverra aussi des notifications pour les réponses à leurs commentaires.

Lorsque FastComments envoie des courriels à vos utilisateurs, nous utiliserons un From Name et une adresse Email par défaut de `FastComments Robot` et `noreply@fastcomments.com`.

Nous utiliserons également notre propre logo dans le pied de page de ces courriels.

Si vous avez FastComments Flex ou Pro, tout cela peut être personnalisé par domaine via la page "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Lorsque vous personnalisez le logo affiché dans les courriels, assurez-vous que la taille que vous téléversez est la même que celle que vous voulez afficher dans le pied de page du courriel.

### Lors de la personnalisation du `From Domain`

Si vous personnalisez le `From Domain`, les fournisseurs et clients de courriel doivent savoir que FastComments est autorisé à envoyer des courriels en votre nom. Sinon,
définir le `From Domain` sans suivre les étapes ci-dessous entraînera probablement l'acheminement des courriels vers le dossier de spam.

#### 1. Configurez SPF

Pour permettre à FastComments d'envoyer des courriels en toute sécurité au nom de votre domaine, assurez-vous d'ajouter un enregistrement SPF qui nous y autorise.

Assurez-vous qu'il existe des enregistrements SPF permettant à `mail.fastcomments.com` et `sib.fastcomments.com` d'envoyer du courrier au nom de votre domaine.

Plus d'informations sur la façon de procéder ici : https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configurez DKIM

En plus du SPF, vous devriez configurer DKIM. Une fois votre configuration DNS prête, vous pouvez cliquer sur "Show Advanced" dans la page de configuration des domaines
pour afficher les paramètres DKIM par domaine.

Vous pouvez également [invoker the API](/guide-api.html#domain-config-structure) pour définir la configuration DKIM.

### Liens de désabonnement

Lorsque vous utilisez SSO, les fonctionnalités de désabonnement utilisées dans les courriels et les notifications peuvent être personnalisées [via the DomainConfigs API](/guide-api.html#domain-config-structure).

### Obfuscation des liens dans les courriels

Si la réputation de domaine de votre site fait en sorte que les courriels de notification atterrissent dans le spam, vous pouvez faire transiter les boutons « view comment » par `fastcomments.com` au lieu de lier directement vers votre page. Les fournisseurs de boîtes aux lettres évaluent chaque lien dans le corps du courriel en fonction de la réputation de la destination, donc lorsque votre domaine est signalé, les liens nus contribuent au score de spam, peu importe la qualité de votre configuration d'envoi.

Activez cela sous "Show Advanced" sur la page My Domains, dans la section "Email Link Obfuscation". Ce paramètre est par domaine.

Lorsqu'il est activé, les liens dans les courriels de type mention, reply, new-comment, subscribed-page, profile-comment et digest sont réécrits en courts jetons qui redirigent vers la page d'origine lors du clic. La destination est liée à votre tenant : la redirection ne transfère que vers des URL dont l'hôte correspond à l'un de vos domaines configurés, et les jetons expirent automatiquement après 30 jours.

L'expérience après clic reste inchangée. Les lecteurs arrivent toujours sur votre page avec le commentaire affiché à l'écran.