Puisque les liens de connexion sont essentiellement des mots de passe, nous prenons la sécurité très au sérieux.

Tous les liens de connexion dans notre système sont configurés pour expirer après une certaine période, et nous disposons également de mécanismes pour détecter la découverte par force brute d'un lien de connexion. Certains liens de connexion sont divisés en plusieurs mots de passe, et si l'un est deviné, l'autre sera invalidé.

### Sécurité comparée aux mots de passe

Avec la plupart des systèmes qui requièrent un mot de passe, vous pouvez utiliser un mécanisme « mot de passe oublié » si vous avez l'adresse courriel de l'utilisateur. Cela signifie que, si vous avez accès au compte courriel de l'utilisateur, peu importe que le système attaqué utilise des mots de passe ou des liens magiques.

### Sécurité comparée à la MFA

Les liens de connexion sont moins sécurisés que la MFA. FastComments prend maintenant en charge l'authentification à deux facteurs (2FA) pour les comptes administrateurs afin d'offrir une sécurité renforcée. Lorsque la 2FA est activée, elle est exigée même lors de l'utilisation des liens de connexion.

---