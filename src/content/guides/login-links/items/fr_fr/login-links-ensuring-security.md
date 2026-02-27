Étant donné que les liens de connexion sont essentiellement des mots de passe, nous prenons la sécurité très au sérieux.

Tous les liens de connexion de notre système sont configurés pour expirer après une certaine période, et nous disposons également de mécanismes pour détecter
le fait de deviner un lien de connexion. Certains liens de connexion sont divisés en plusieurs mots de passe, et si l'un est deviné,
les autres seront invalidés.

### Sécurité par rapport aux mots de passe

Avec la plupart des systèmes qui nécessitent un mot de passe, vous pouvez passer par un mécanisme Mot de passe oublié
si vous disposez de l'e-mail de l'utilisateur. Cela signifie que, si vous avez accès au compte e-mail de l'utilisateur,
il n'y a pas de différence importante que le système attaqué utilise des mots de passe ou des liens magiques.

### Alertes de connexion depuis une nouvelle IP

Lorsqu'une connexion se produit depuis une adresse IP qui n'a pas été vue auparavant pour un compte donné, FastComments envoie un e-mail d'alerte de sécurité
avec la localisation approximative et l'adresse IP. Cela aide les utilisateurs à détecter les accès non autorisés. Notez que FastComments ne stocke pas
les adresses IP brutes — seule une forme obfusquée est conservée pour des raisons de sécurité.

### E-mail de secours pour la récupération de compte

Si vous perdez l'accès à votre e-mail principal, vous pouvez utiliser un e-mail de secours vérifié pour récupérer votre compte. Votre e-mail de secours fonctionne
avec tous les flux de connexion. Vous pouvez le saisir sur la page Nom d'utilisateur oublié, l'utiliser avec la connexion par lien magique, ou le taper dans le
champ nom d'utilisateur/e-mail pour la connexion par mot de passe.

Pour configurer un e-mail de secours, allez dans [Détails du compte](https://fastcomments.com/auth/my-account/edit-details) et cliquez
**Définir un e-mail de secours**. Votre e-mail de secours est uniquement utilisé pour la récupération de compte et ne recevra pas de notifications.

### Sécurité par rapport à l'authentification multifacteur (MFA)

Les liens de connexion sont moins sécurisés que l'authentification multifacteur (MFA). FastComments prend désormais en charge l'authentification à deux facteurs (2FA)
pour les comptes administrateur afin d'offrir une sécurité renforcée. Lorsque l'authentification à deux facteurs (2FA) est activée, elle est requise même lors de l'utilisation des liens de connexion.