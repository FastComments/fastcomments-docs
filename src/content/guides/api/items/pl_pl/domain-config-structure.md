A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Struktura konfiguracji domeny'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Domenа, nie URL, np. "fastcomments.com" lub "www.example.com". Można uwzględnić subdomenę, jeśli chcesz ograniczyć do subdomeny. Maks. 1000 znaków. **/
    domain: string
    /** Nazwa nadawcy używana przy wysyłaniu e-maili. **/
    emailFromName?: string
    /** Adres e-mail nadawcy używany przy wysyłaniu wiadomości. Upewnij się, że SPF jest skonfigurowany, aby pozwolić mail.fastcomments.com wysyłać wiadomości w imieniu domeny użytej w tym atrybucie. **/
    emailFromEmail?: string
    /** TYLKO DO ODCZYTU. Kiedy obiekt został utworzony. **/
    createdAt: string
    /** Logo powiązane z tą domeną. Używane w e-mailach. Użyj HTTPS. **/
    logoSrc?: string
    /** Mniejsze logo powiązane z tą domeną. Użyj HTTPS. **/
    logoSrc100px?: string
    /** TYLKO SSO. URL używany w stopce każdej wysyłanej wiadomości e-mail. Obsługuje zmienną "[userId]". **/
    footerUnsubscribeURL?: string
    /** TYLKO SSO. Nagłówki używane w każdej wysyłanej wiadomości e-mail. Przydatne na przykład do ustawiania nagłówków związanych z rezygnacją z subskrypcji w celu poprawy dostarczalności. Wpis List-Unsubscribe w tym rekordzie, jeśli istnieje, obsługuje zmienną "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Wyłącz wszystkie linki rezygnacji z subskrypcji. Niezalecane, może negatywnie wpłynąć na wskaźniki dostarczalności. **/
    disableUnsubscribeLinks?: boolean
    /** Konfiguracja DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura konfiguracji DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Nazwa domeny w rekordzie DKIM. **/
    domainName: string
    /** Selektor klucza DKIM do użycia. **/
    keySelector: string
    /** Twój klucz prywatny. Zaczyna się od -----BEGIN PRIVATE KEY----- i kończy na -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Do uwierzytelniania

Konfiguracja domeny jest używana do określenia, które strony mogą hostować widget FastComments dla Twojego konta. Jest to podstawowa forma
uwierzytelniania, co oznacza, że dodanie lub usunięcie konfiguracji domen może wpłynąć na dostępność Twojej instalacji FastComments
w środowisku produkcyjnym.

Don't remove or update the `domain` property of a `Domain Config` for a domain that is currently in use unless disabling that domain is intended.

This has the same behavior as removing a domain from [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Also note that removing a domain from the `My Domains` UI will remove any corresponding configuration for that domain that may have been added via this UI.

### Dostosowywanie e-maili

The unsubscribe link in the email footer, and the one-click-unsubscribe feature offered by many email clients, can be configured via this API by defining `footerUnsubscribeURL` and `emailHeaders`, respectively.

### DKIM

Po zdefiniowaniu rekordów DNS DKIM, wystarczy zaktualizować obiekt DomainConfig o konfigurację DKIM, używając podanej struktury.

---