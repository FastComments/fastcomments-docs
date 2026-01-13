Un objet `DomainConfig` représente la configuration d'un domaine pour un locataire.

La structure de l'objet `DomainConfig` est la suivante :

[inline-code-attrs-start title = 'Structure de Config de Domaine'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** A domain, not a URL, like "fastcomments.com" or "www.example.com". Subdomain may be included if limiting to a subdomain is desired. Max 1000 characters. **/
    domain: string
    /** The From-Name used when sending emails. **/
    emailFromName?: string
    /** The From-Email used when sending emails. Ensure SPF is setup to allow mail.fastcomments.com to send emails as the domain used in this attribute. **/
    emailFromEmail?: string
    /** READONLY. When the object was created. **/
    createdAt: string
    /** The logo related to this domain. Used in emails. Use HTTPS. **/
    logoSrc?: string
    /** A smaller logo related to this domain. Use HTTPS. **/
    logoSrc100px?: string
    /** SSO ONLY. The URL used in the footer of every email sent. Supports a "[userId]" variable. **/
    footerUnsubscribeURL?: string
    /** SSO ONLY. The headers used in of every email sent. Useful for example for setting unsubscribe related headers to improve delivery. The List-Unsubscribe entry in this Record, if it exists, supports a "[userId]" variable. **/
    emailHeaders?: Record<string, string>
    /** Disable all unsubscribe links. Not recommended, may hurt delivery rates. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM Configuration. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Config DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** The domain name in your DKIM record. **/
    domainName: string
    /** The DKIM key selector to use. **/
    keySelector: string
    /** Your private key. Start with -----BEGIN PRIVATE KEY----- and end with -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Pour l'Authentification

La Configuration de Domaine est utilisée pour déterminer quels sites peuvent héberger le widget FastComments pour votre compte. C'est une forme basique
d'authentification, ce qui signifie qu'ajouter ou supprimer des Configurations de Domaine peut impacter la disponibilité de votre installation FastComments
en production.

Ne supprimez pas et ne mettez pas à jour la propriété `domain` d'une `Config de Domaine` pour un domaine actuellement en cours d'utilisation, sauf si la désactivation de ce domaine est intentionnelle.

Cela a le même comportement que supprimer un domaine de [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Notez également que supprimer un domaine de l'interface utilisateur `Mes Domaines` supprimera toute configuration correspondante pour ce domaine qui aurait pu être ajoutée via cette interface.

### Pour la Personnalisation des Emails

Le lien de désabonnement dans le pied de page des emails, et la fonctionnalité de désabonnement en un clic offerte par de nombreux clients de messagerie, peuvent être configurés via cette API en définissant `footerUnsubscribeURL` et `emailHeaders`, respectivement.

### Pour DKIM

Après avoir défini vos enregistrements DNS DKIM, mettez simplement à jour le DomainConfig avec votre configuration DKIM en utilisant la structure définie.
