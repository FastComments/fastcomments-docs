### Przegląd

FastComments Collab Chat rozszerza standardowy widget komentarzy FastComments, więc dziedziczy wszystkie opcje konfiguracji z bazowego widgetu, dodając jednocześnie kilka opcji specyficznych dla adnotacji tekstowych.

### Wymagana konfiguracja

#### tenantId

Wymagany jest Twój FastComments Tenant ID. Możesz go znaleźć w swoim [panelu FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Opcje specyficzne dla Collab Chat

#### urlId

Domyślnie Collab Chat generuje unikalny identyfikator dla każdej rozmowy na podstawie adresu URL strony, ścieżki DOM do elementu oraz wybranego zakresu tekstu. Możesz to nadpisać własnym `urlId`.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Jest to przydatne, gdy struktura Twoich adresów URL może się zmienić, ale chcesz zachować te same rozmowy, lub gdy chcesz udostępniać adnotacje na wielu stronach.

#### topBarTarget

Kontroluje wyświetlanie górnego paska, który pokazuje liczbę użytkowników i liczbę dyskusji. Ustaw na `null`, aby całkowicie wyłączyć górny pasek, lub podaj element DOM, aby wyrenderować go w określonym miejscu.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Włącza stylizację trybu ciemnego, gdy Twoja strona ma ciemne tło. Wykrywanie jest automatyczne, ale czasem warto je nadpisać.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Funkcja zwrotna wywoływana za każdym razem, gdy zmienia się liczba komentarzy. Jest to przydatne do aktualizacji elementów interfejsu, takich jak plakietki czy tytuły stron.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Odziedziczone opcje konfiguracji

Ponieważ Collab Chat rozszerza standardowy widget komentarzy, możesz użyć dowolnej opcji konfiguracji z bazowego widgetu FastComments. Oto niektóre z najczęściej używanych opcji:

#### locale

Ustawia język interfejsu widgetu. FastComments obsługuje dziesiątki języków.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Ustawia wszystkie rozmowy jako tylko do odczytu. Użytkownicy mogą przeglądać istniejące adnotacje, ale nie mogą tworzyć nowych ani odpowiadać.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso i simpleSSO

Integracja z Twoim systemem uwierzytelniania przy użyciu Single Sign-On.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Zobacz dokumentację SSO, aby uzyskać pełne informacje o opcjach uwierzytelniania.

#### maxReplyDepth

Kontroluje, na ile poziomów w głąb mogą schodzić odpowiedzi. Domyślnie Collab Chat ustawia tę wartość na 0, co oznacza, że wszystkie komentarze są płaskie (bez zagnieżdżonych odpowiedzi). Możesz to zmienić, jeśli chcesz mieć wątkowe rozmowy.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Konfiguracja wewnętrzna

Te opcje są ustawiane automatycznie przez Collab Chat i nie powinny być nadpisywane:

`productId` jest automatycznie ustawiane na `3` dla Collab Chat. Rozszerzenie `floating-chat` jest automatycznie ładowane, aby zapewnić funkcjonalność okna czatu. Widget automatycznie wykrywa urządzenia mobilne (ekrany o szerokości poniżej 768px) i odpowiednio dostosowuje interfejs.

### Kompletny przykład

Oto przykład pokazujący kilka opcji konfiguracji razem:

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

Aby uzyskać pełną listę wszystkich dostępnych opcji konfiguracji odziedziczonych z bazowego widgetu, zobacz główną dokumentację konfiguracji FastComments.