`User` est un objet qui représente un dénominateur commun de tous les utilisateurs.

Gardez à l'esprit qu'à FastComments nous avons un tas de cas d'utilisation différents pour les utilisateurs :

- SSO Sécurisé
- SSO Simple
- Utilisateurs Locataires (Par exemple : Administrateurs)
- Commentateurs

Cette API est pour les **Commentateurs** et les utilisateurs créés via **SSO Simple**. En gros, tout utilisateur créé
via votre site peut être accédé via cette API. Les utilisateurs locataires peuvent également être récupérés de cette façon, mais vous obtiendrez plus d'informations en interagissant avec l'API `/tenant-users/`.

Pour le `SSO Sécurisé` veuillez utiliser l'API `/sso-users/`.

Vous ne pouvez pas mettre à jour ces types d'utilisateurs. Ils ont créé leur compte via votre site, donc nous fournissons un accès en lecture seule basique, mais
vous ne pouvez pas faire de modifications. Si vous voulez avoir ce type de flux - vous devez configurer le `SSO Sécurisé`.

La structure de l'objet `User` est la suivante :

[inline-code-attrs-start title = 'Structure de User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** This is also the id used as userId on comment objects. **/
    id: string
    username: string
    /** A link to the commenter's blog, for example. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]
