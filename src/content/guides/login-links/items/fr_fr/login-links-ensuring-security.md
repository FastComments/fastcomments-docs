Étant donné que les liens de connexion sont essentiellement des mots de passe, nous prenons la sécurité très au sérieux.

Tous les liens de connexion de notre système expirent après une certaine durée, et nous disposons également de mécanismes pour détecter la tentative de deviner un lien de connexion. Certains liens de connexion sont répartis en plusieurs mots de passe, et si l'un est deviné, les autres seront invalidés.

### Sécurité par rapport aux mots de passe

Dans la plupart des systèmes nécessitant un mot de passe, il suffit d'utiliser la procédure « Mot de passe oublié » si vous disposez de l'adresse e-mail de l'utilisateur. Cela signifie que si vous avez accès au compte e-mail de l'utilisateur, il est indifférent que le système attaqué utilise des mots de passe ou des liens magiques.

### Alertes de connexion depuis une nouvelle IP

Lorsqu'une connexion est effectuée depuis une adresse IP qui n'a pas été vue auparavant pour un compte donné, FastComments envoie un e-mail d'alerte de sécurité contenant la localisation approximative et l'adresse IP. Cela aide les utilisateurs à détecter les accès non autorisés. Notez que FastComments ne stocke pas les adresses IP brutes — seule une forme obfusquée est conservée pour des raisons de sécurité.

### Sécurité par rapport à la MFA

Les liens de connexion sont moins sécurisés que la MFA. FastComments prend désormais en charge l'authentification à deux facteurs (2FA) pour les comptes administrateur afin d'offrir une sécurité renforcée. Lorsque la 2FA est activée, elle est requise même lors de l'utilisation de liens de connexion.

---