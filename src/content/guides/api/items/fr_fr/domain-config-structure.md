Un objet `DomainConfig` représente la configuration d'un domaine pour un tenant.

La structure de l'objet `DomainConfig` est la suivante :

[inline-code-attrs-start title = 'Structure de l’objet DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Un domaine, pas une URL, comme "fastcomments.com" ou "www.example.com". Le sous-domaine peut être inclus si vous souhaitez limiter à un sous-domaine. Maximum 1000 caractères. **/
    domain: string
    /** Le From-Name utilisé lors de l'envoi des e-mails. **/
    emailFromName?: string
    /** Le From-Email utilisé lors de l'envoi des e-mails. Assurez-vous que le SPF est configuré pour permettre à mail.fastcomments.com d'envoyer des e-mails au nom du domaine utilisé dans cet attribut. **/
    emailFromEmail?: string
    /** EN LECTURE SEULE. Date de création de l'objet. **/
    createdAt: string
    /** Le logo lié à ce domaine. Utilisé dans les e-mails. Utilisez HTTPS. **/
    logoSrc?: string
    /** Un logo plus petit lié à ce domaine. Utilisez HTTPS. **/
    logoSrc100px?: string
    /** SSO UNIQUEMENT. L'URL utilisée dans le pied de page de chaque e-mail envoyé. Prend en charge une variable "[userId]". **/
    footerUnsubscribeURL?: string
    /** SSO UNIQUEMENT. Les en-têtes utilisés dans chaque e-mail envoyé. Utile par exemple pour définir des en-têtes liés à la désinscription afin d'améliorer la délivrabilité. L'entrée List-Unsubscribe dans cet enregistrement, si elle existe, prend en charge la variable "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Désactiver tous les liens de désinscription. Non recommandé, peut nuire aux taux de délivrabilité. **/
    disableUnsubscribeLinks?: boolean
    /** Configuration DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de configuration DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Le nom de domaine dans votre enregistrement DKIM. **/
    domainName: string
    /** Le sélecteur de clé DKIM à utiliser. **/
    keySelector: string
    /** La clé publique, au format PEM. Renvoyée dans les réponses GET. **/
    publicKey: string
    /** @deprecated N'est plus renvoyé dans les réponses de l'API. Accepté en écriture pour compatibilité rétroactive. **/
    privateKey?: string
}
[inline-code-end]

### Pour l'authentification

Domain Configuration est utilisé pour déterminer quels sites peuvent héberger le widget FastComments pour votre compte. Il s'agit d'une forme basique
d'authentification, ce qui signifie que l'ajout ou la suppression de toute configuration de domaine peut affecter la disponibilité de votre installation FastComments
en production.

Ne supprimez pas et ne mettez pas à jour la propriété `domain` d'un `Domain Config` pour un domaine actuellement utilisé, sauf si l'intention est de désactiver ce domaine.

Cela a le même comportement que la suppression d'un domaine depuis [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Notez également que la suppression d'un domaine depuis l'interface `My Domains` supprimera toute configuration correspondante pour ce domaine qui aurait pu être ajoutée via cette interface.

### Pour la personnalisation des e-mails

Le lien de désinscription dans le pied de page de l'e-mail, ainsi que la fonctionnalité de désinscription en un clic offerte par de nombreux clients de messagerie, peuvent être configurés via cette API en définissant `footerUnsubscribeURL` et `emailHeaders`, respectivement.

### Pour DKIM

Après avoir défini vos enregistrements DNS DKIM, mettez simplement à jour le DomainConfig avec votre configuration DKIM en utilisant la structure définie.