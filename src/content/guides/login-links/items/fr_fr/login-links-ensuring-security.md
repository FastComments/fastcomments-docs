Étant donné que les liens de connexion sont essentiellement des mots de passe, nous prenons la sécurité très au sérieux.

Tous les liens de connexion de notre système expirent après une certaine période, et nous disposons également de mécanismes en place pour détecter
les tentatives de deviner un lien de connexion. Certains liens de connexion sont divisés en plusieurs mots de passe, et si l'un est deviné,
l'autre sera invalidé.

### Sécurité comparée aux mots de passe

Dans la plupart des systèmes qui exigent un mot de passe, vous pouvez passer par un mécanisme « Mot de passe oublié »
si vous disposez de l'email de l'utilisateur. Cela signifie que, si vous avez accès au compte email de l'utilisateur,
il n'importe pas si le système attaqué utilise des mots de passe ou des liens magiques.

### Sécurité comparée à l'authentification multifacteur (MFA)

Les liens de connexion sont moins sécurisés que l'authentification multifacteur (MFA). FastComments prend désormais en charge l'authentification à deux facteurs (2FA)
pour les comptes administrateur afin d'offrir une sécurité renforcée. Lorsque la 2FA est activée, elle est requise même lors de l'utilisation de liens de connexion.