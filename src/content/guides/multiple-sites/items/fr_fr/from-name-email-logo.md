---
Parfois, FastComments doit envoyer des e‑mails à vos utilisateurs, surtout si vous n’utilisez pas le SSO sécurisé.

Cela inclut, par exemple, la vérification de leur compte ou de leur activité lors du premier commentaire. FastComments leur enverra également des notifications pour les réponses à leurs commentaires.

Lorsque FastComments envoie des e‑mails à vos utilisateurs, nous utilisons par défaut un nom d’expéditeur et une adresse e‑mail de `FastComments Robot` et `noreply@fastcomments.com`.

Nous utiliserons également notre propre logo dans le pied de page de ces e‑mails.

Si vous disposez de FastComments Flex ou Pro, tout cela peut être personnalisé par domaine via la page « My Domains » :

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Formulaire de paramètres d\'e-mail par domaine avec les champs Nom d\'expéditeur, E-mail d\'expéditeur et téléchargement du logo'; title='Personnalisation du nom d\'expéditeur, de l\'e-mail et du logo' app-screenshot-end]

Lorsque vous personnalisez le logo affiché dans les e‑mails, assurez‑vous que la taille que vous téléchargez correspond à la taille que vous souhaitez afficher dans le pied de page de l’e‑mail.

### Lors de la personnalisation du `From Domain`

Si vous personnalisez le `From Domain`, les fournisseurs de messagerie et les clients doivent savoir que FastComments est autorisé à envoyer des e‑mails en votre nom. Sinon, définir le `From Domain` sans suivre les étapes ci‑dessous entraînera probablement que les e‑mails soient classés comme spam.

#### 1. Configurer SPF

Pour permettre à FastComments d’envoyer des e‑mails en toute sécurité au nom de votre domaine, assurez‑vous d’ajouter un enregistrement SPF qui nous autorise à le faire.

Assurez‑vous qu’il existe des enregistrements SPF autorisant `mail.fastcomments.com` et `sib.fastcomments.com` à envoyer des e‑mails au nom de votre domaine.

Vous trouverez plus d’informations sur la façon de procéder ici : https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configurer DKIM

En plus du SPF, vous devez configurer DKIM. Une fois votre configuration DNS prête, vous pouvez cliquer sur « Show Advanced » dans la page de configuration des domaines pour afficher les paramètres DKIM par domaine.

Vous pouvez également [invoker l’API](/guide-api.html#domain-config-structure) pour définir la configuration DKIM.

### Liens de désabonnement

Lors de l’utilisation du SSO, les fonctionnalités de désabonnement utilisées dans les e‑mails et les notifications peuvent être personnalisées [via l’API DomainConfigs](/guide-api.html#domain-config-structure).

### Obfuscation des liens d’e‑mail

Si la réputation du domaine de votre site entraîne le placement des e‑mails de notification dans le spam, vous pouvez rediriger les boutons « view comment » via `fastcomments.com` au lieu de créer un lien direct vers votre page. Les fournisseurs de boîtes aux lettres évaluent chaque lien dans le corps de l’e‑mail en fonction de la réputation de la destination, de sorte que lorsque votre domaine est signalé, les liens directs augmentent le score de spam, quel que soit la propreté de votre configuration d’envoi.

Activez cette option sous « Show Advanced » sur la page My Domains, dans la section « Email Link Obfuscation ». Ce paramètre est appliqué par domaine.

Lorsqu’elle est activée, les liens dans les e‑mails de mention, de réponse, de nouveau commentaire, de page abonnée, de commentaire de profil et de digest sont réécrits en courts jetons qui redirigent vers la page d’origine au clic. La destination est liée à votre locataire : la redirection ne renvoie que vers les URL dont l’hôte correspond à l’un de vos domaines configurés, et les jetons expirent automatiquement après 30 jours.

L’expérience de navigation reste inchangée. Les lecteurs arrivent toujours sur votre page avec le commentaire affiché à l’écran.

---