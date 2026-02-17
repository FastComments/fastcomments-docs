Un objet `DomainConfig` représente la configuration d'un domaine pour un locataire.

La structure de l'objet `DomainConfig` est la suivante :

[inline-code-attrs-start title = 'Structure de Config de Domaine'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Un domaine, pas une URL, comme "fastcomments.com" ou "www.example.com". Le sous-domaine peut être inclus si l'on souhaite limiter à un sous-domaine. Maximum 1000 caractères. **/
    domain: string
    /** Le nom d'expéditeur utilisé lors de l'envoi d'e-mails. **/
    emailFromName?: string
    /** L'adresse e-mail d'expéditeur utilisée lors de l'envoi d'e-mails. Assurez-vous que le SPF est configuré pour permettre à mail.fastcomments.com d'envoyer des e-mails au nom du domaine utilisé dans cet attribut. **/
    emailFromEmail?: string
    /** EN LECTURE SEULE. Date de création de l'objet. **/
    createdAt: string
    /** Le logo lié à ce domaine. Utilisé dans les e-mails. Utiliser HTTPS. **/
    logoSrc?: string
    /** Un logo plus petit lié à ce domaine. Utiliser HTTPS. **/
    logoSrc100px?: string
    /** SSO UNIQUEMENT. L'URL utilisée dans le pied de page de chaque e-mail envoyé. Prend en charge la variable "[userId]". **/
    footerUnsubscribeURL?: string
    /** SSO UNIQUEMENT. Les en-têtes utilisés dans chaque e-mail envoyé. Utile, par exemple, pour définir des en-têtes liés à la désinscription afin d'améliorer la délivrabilité. L'entrée List-Unsubscribe dans cet enregistrement, si elle existe, prend en charge la variable "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Désactiver tous les liens de désinscription. Non recommandé, peut nuire aux taux de délivrabilité. **/
    disableUnsubscribeLinks?: boolean
    /** Configuration DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la configuration DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Le nom de domaine dans votre enregistrement DKIM. **/
    domainName: string
    /** Le sélecteur de clé DKIM à utiliser. **/
    keySelector: string
    /** La clé publique, au format PEM. Renvoyée dans les réponses GET. **/
    publicKey: string
    /** @deprecated N'est plus renvoyée dans les réponses de l'API. Acceptée en écriture pour compatibilité ascendante. **/
    privateKey?: string
}
[inline-code-end]

### Pour l'authentification

La configuration de domaine est utilisée pour déterminer quels sites peuvent héberger le widget FastComments pour votre compte. Il s'agit d'une forme d'authentification basique, ce qui signifie que l'ajout ou la suppression de configurations de domaine peut affecter la disponibilité de votre installation FastComments en production.

Ne supprimez pas et ne mettez pas à jour la propriété `domain` d'un `Domain Config` pour un domaine actuellement utilisé, sauf si vous avez l'intention de désactiver ce domaine.

Cela a le même comportement que la suppression d'un domaine depuis [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Notez également que la suppression d'un domaine depuis l'interface `My Domains` supprimera toute configuration correspondante pour ce domaine ayant pu être ajoutée via cette interface.

### Pour la personnalisation des courriels

Le lien de désinscription dans le pied de page du courriel, ainsi que la fonction de désinscription en un clic proposée par de nombreux clients de messagerie, peuvent être configurés via cette API en définissant respectivement `footerUnsubscribeURL` et `emailHeaders`.

### Pour DKIM

Après avoir défini vos enregistrements DNS DKIM, mettez simplement à jour le DomainConfig avec votre configuration DKIM en utilisant la structure définie. 

---