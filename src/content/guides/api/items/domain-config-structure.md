A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Domain Config Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** A domain, not a URL, like "fastcomments.com" or "www.example.com". Subdomain may be included if limiting to a subdomain is desired. Max 1000 characters. **/
    domain: string;
    /** The From-Name used when sending emails. **/
    emailFromName?: string;
    /** The From-Email used when sending emails. Ensure SPF is setup to allow mail.fastcomments.com to send emails as the domain used in this attribute. **/
    emailFromEmail?: string;
    /** READONLY. When the object was created. **/
    createdAt: string;
    /** The logo related to this domain. Used in emails. Use HTTPS. **/
    logoSrc?: string;
    /** A smaller logo related to this domain. Use HTTPS. **/
    logoSrc100px?: string;
    /** SSO ONLY. The URL used in the footer of every email sent. Supports a "[userId]" variable. **/
    footerUnsubscribeURL?: string;
    /** SSO ONLY. The headers used in of every email sent. Useful for example for setting unsubscribe related headers to improve delivery. The List-Unsubscribe entry in this Record, if it exists, supports a "[userId]" variable. **/
    emailHeaders?: Record<string, string>;
    /** Disable all unsubscribe links. Not recommended, may hurt delivery rates. **/
    disableUnsubscribeLinks?: boolean;
    /** DKIM Configuration. **/
    dkim?: DomainConfigDKIM;
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM Config Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** The domain name in your DKIM record. **/
    domainName: string;
    /** The DKIM key selector to use. **/
    keySelector: string;
    /** Your private key. Start with -----BEGIN PRIVATE KEY----- and end with -----END PRIVATE KEY----- **/
    privateKey: string;
}
[inline-code-end]

### For Authentication

Domain Configuration is used to determine which sites can host the FastComments widget for your account. This is a basic form
of authentication, meaning adding or removing any Domain Configurations can impact the availability of your FastComments installation
in production.

Don't remove or update the `domain` property of a `Domain Config` for a domain that is currently in use unless disabling that domain is intended.

This has the same behavior as removing a domain from [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Also note that removing a domain from the `My Domains` UI will remove any corresponding configuration for that domain that may have been added via this UI.

### For Email Customization

The unsubscribe link in the email footer, and the one-click-unsubscribe feature offered by many email clients, can be configured via this API by defining `footerUnsubscribeURL` and `emailHeaders`, respectively.

### For DKIM

After defining your DKIM DNS records, simply update the DomainConfig with your DKIM configuration using the defined structure. 
