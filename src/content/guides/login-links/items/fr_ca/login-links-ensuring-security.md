Puisque les liens de connexion sont essentiellement des mots de passe, nous prenons la sécurité très au sérieux.

Tous les liens de connexion de notre système sont configurés pour expirer après une certaine période, et nous disposons aussi de mécanismes pour détecter la tentative de deviner un lien de connexion. Certains liens de connexion sont divisés en plusieurs mots de passe, et si l'un est deviné, les autres seront invalidés.

### Sécurité comparée aux mots de passe

Avec la plupart des systèmes qui exigent un mot de passe, vous pouvez passer par un mécanisme « Mot de passe oublié » si vous avez l'adresse courriel de l'utilisateur. Cela signifie que si vous avez accès au compte courriel de l'utilisateur, il est indifférent pour un attaquant que le système utilise des mots de passe ou des liens de connexion.

### Alertes de connexion depuis une nouvelle adresse IP

Lorsqu'une connexion provient d'une adresse IP qui n'a pas été vue auparavant pour un compte donné, FastComments envoie un courriel d'alerte de sécurité avec la localisation approximative et l'adresse IP. Cela aide les utilisateurs à détecter des accès non autorisés. Notez que FastComments ne stocke pas les adresses IP en clair — seule une forme obfusquée est conservée pour des raisons de sécurité.

### Sécurité comparée au MFA

Les liens de connexion sont moins sécurisés que le MFA. FastComments prend maintenant en charge l'authentification à deux facteurs (2FA) pour les comptes administrateur afin d'offrir une sécurité renforcée. Lorsque la 2FA est activée, elle est requise même lors de l'utilisation de liens de connexion.