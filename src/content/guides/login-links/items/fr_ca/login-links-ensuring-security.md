---
Parce que les liens de connexion sont essentiellement des mots de passe, nous prenons la sécurité très au sérieux.

Tous les liens de connexion de notre système sont configurés pour expirer après une certaine période, et nous disposons également de mécanismes
pour détecter la devinette d'un lien de connexion. Certains liens de connexion sont divisés en plusieurs mots de passe, et si l'un est deviné,
l'autre sera invalidé.

### Sécurité comparée à l'authentification par mot de passe

Avec la plupart des systèmes qui exigent un mot de passe, vous pouvez passer par un mécanisme « Mot de passe oublié »
si vous avez l'adresse courriel de l'utilisateur. Cela signifie que, si vous avez accès au compte courriel de l'utilisateur,
peu importe que le système attaqué utilise des mots de passe ou des liens magiques.

### Alertes de connexion depuis une nouvelle IP

Lorsqu'une connexion a lieu depuis une adresse IP qui n'a pas été vue auparavant pour un compte donné, FastComments envoie un courriel d'alerte de sécurité
avec l'emplacement approximatif et l'adresse IP. Cela aide les utilisateurs à détecter des accès non autorisés. Notez que FastComments ne stocke pas
les adresses IP brutes — seule une forme obfusquée est conservée à des fins de sécurité.

### Courriel de secours pour la récupération du compte

Si vous perdez l'accès à votre courriel principal, vous pouvez utiliser un courriel de secours vérifié pour récupérer votre compte. Votre courriel de secours fonctionne
avec tous les flux de connexion. Vous pouvez l'entrer sur la page « Nom d'utilisateur oublié », l'utiliser pour la connexion par lien magique, ou le saisir dans le
champ nom d'utilisateur/courriel pour la connexion par mot de passe.

Pour configurer un courriel de secours, allez à [Détails du compte](https://fastcomments.com/auth/my-account/edit-details) et cliquez
**Définir un courriel de secours**. Votre courriel de secours est uniquement utilisé pour la récupération de compte et ne recevra pas de notifications.

### Sécurité comparée à l'authentification multifacteur (MFA)

Les liens de connexion sont moins sécurisés que l'authentification multifacteur (MFA). FastComments prend désormais en charge l'authentification à deux facteurs (2FA)
pour les comptes d'administrateur afin d'offrir une sécurité renforcée. Lorsque la 2FA est activée, elle est requise même lors de l'utilisation de liens de connexion.

---