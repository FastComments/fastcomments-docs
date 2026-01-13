SAML (Security Assertion Markup Language) to oparty na XML otwarty standard do wymiany danych uwierzytelniania i autoryzacji między stronami, 
w szczególności między dostawcą tożsamości (IdP) a dostawcą usług (SP).

### Jak działa SAML

SAML umożliwia pojedyncze logowanie (SSO) poprzez pozwolenie użytkownikom na jednokrotne uwierzytelnienie się u dostawcy tożsamości, a następnie dostęp do wielu aplikacji 
bez ponownego wprowadzania poświadczeń. Gdy użytkownik próbuje uzyskać dostęp do FastComments:

1. **Żądanie uwierzytelnienia**: FastComments przekierowuje użytkownika do twojego dostawcy tożsamości
2. **Uwierzytelnianie użytkownika**: Użytkownik uwierzytelnia się w twoim IdP (np. Active Directory, Okta, Azure AD)
3. **Odpowiedź SAML**: IdP wysyła podpisane stwierdzenie SAML z powrotem do FastComments
4. **Dostęp użytkownika**: FastComments weryfikuje stwierdzenie i przyznaje dostęp uwierzytelnionemu użytkownikowi

### Korzyści z SAML

- **Zwiększone bezpieczeństwo**: Zcentralizowane uwierzytelnianie zmniejsza ryzyka związane z hasłami
- **Lepsze doświadczenie użytkownika**: Użytkownicy logują się raz i bezproblemowo uzyskują dostęp do wielu aplikacji
- **Zgodność z przepisami**: Pomaga spełnić wymogi regulacyjne dotyczące kontroli dostępu i ścieżek audytu
- **Kontrola administracyjna**: Administratorzy IT utrzymują scentralizowane zarządzanie użytkownikami

### Obsługa SAML 2.0

FastComments implementuje SAML 2.0, najpowszechniej stosowaną wersję standardu SAML. Nasza implementacja obsługuje:

- Wiązania HTTP-POST i HTTP-Redirect
- Podpisane odpowiedzi i stwierdzenia SAML
- Zaszyfrowane stwierdzenia (opcjonalnie)
- Wiele algorytmów podpisu i skrótu
- Różne formaty identyfikatorów nazw