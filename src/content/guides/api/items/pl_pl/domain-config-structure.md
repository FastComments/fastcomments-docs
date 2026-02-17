Obiekt `DomainConfig` reprezentuje konfigurację dla domeny dla najemcy.

Struktura obiektu `DomainConfig` jest następująca:

[inline-code-attrs-start title = 'Struktura konfiguracji domeny'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Domena, nie URL, na przykład "fastcomments.com" lub "www.example.com". Można uwzględnić subdomenę, jeśli chcesz ograniczyć do subdomeny. Maksymalnie 1000 znaków. **/
    domain: string
    /** Nazwa nadawcy używana przy wysyłaniu e-maili. **/
    emailFromName?: string
    /** Adres e-mail nadawcy używany przy wysyłaniu e-maili. Upewnij się, że SPF jest skonfigurowany, aby pozwolić mail.fastcomments.com wysyłać e-maile jako domena użyta w tym atrybucie. **/
    emailFromEmail?: string
    /** TYLKO DO ODCZYTU. Kiedy obiekt został utworzony. **/
    createdAt: string
    /** Logo powiązane z tą domeną. Używane w e-mailach. Używaj HTTPS. **/
    logoSrc?: string
    /** Mniejsze logo powiązane z tą domeną. Używaj HTTPS. **/
    logoSrc100px?: string
    /** TYLKO SSO. URL używany w stopce każdego wysyłanego e-maila. Obsługuje zmienną "[userId]". **/
    footerUnsubscribeURL?: string
    /** TYLKO SSO. Nagłówki używane w każdym wysyłanym e-mailu. Przydatne na przykład do ustawiania nagłówków związanych z rezygnacją z subskrypcji w celu poprawy dostarczalności. Wpis List-Unsubscribe w tym rekordzie, jeśli istnieje, obsługuje zmienną "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Wyłącz wszystkie linki do rezygnacji z subskrypcji. Niewskazane, może negatywnie wpłynąć na wskaźniki dostarczalności. **/
    disableUnsubscribeLinks?: boolean
    /** Konfiguracja DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura konfiguracji DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Nazwa domeny w Twoim rekordzie DKIM. **/
    domainName: string
    /** Selektor klucza DKIM do użycia. **/
    keySelector: string
    /** Klucz publiczny w formacie PEM. Zwracany w odpowiedziach GET. **/
    publicKey: string
    /** @deprecated Nie jest już zwracany w odpowiedziach API. Akceptowany przy zapisie dla zachowania zgodności wstecznej. **/
    privateKey?: string
}
[inline-code-end]

### Do uwierzytelniania

Konfiguracja domeny jest używana do określenia, które witryny mogą hostować widget FastComments dla Twojego konta. Jest to podstawowa forma
uwierzytelniania, co oznacza, że dodanie lub usunięcie jakiejkolwiek konfiguracji domeny może wpłynąć na dostępność Twojej instalacji FastComments
w produkcji.

Nie usuwaj ani nie aktualizuj właściwości `domain` obiektu `Domain Config` dla domeny, która jest obecnie używana, chyba że zamierzasz wyłączyć tę domenę.

Ma to takie samo zachowanie jak usunięcie domeny z [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Zwróć również uwagę, że usunięcie domeny z interfejsu `My Domains` spowoduje usunięcie wszelkich odpowiadających konfiguracji dla tej domeny, które mogły zostać dodane za pomocą tego interfejsu.

### Dla personalizacji e-maili

Link do rezygnacji z subskrypcji w stopce e-maila oraz funkcja jednego kliknięcia do rezygnacji oferowana przez wielu klientów pocztowych mogą być skonfigurowane przez to API poprzez zdefiniowanie odpowiednio `footerUnsubscribeURL` i `emailHeaders`.

### Dla DKIM

Po zdefiniowaniu rekordów DKIM w DNS po prostu zaktualizuj obiekt DomainConfig o konfigurację DKIM, używając podanej struktury.

---