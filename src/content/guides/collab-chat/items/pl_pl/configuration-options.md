### Przegląd

FastComments Collab Chat rozszerza standardowy widżet komentarzy FastComments, dzięki czemu dziedziczy wszystkie opcje konfiguracyjne z podstawowego widżetu, jednocześnie dodając kilka specyficznych dla adnotacji tekstowych.

### Wymagana konfiguracja

#### tenantId

Wymagane jest Twoje Tenant ID FastComments. Możesz je znaleźć w swoim [panelu FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Opcje specyficzne dla Collab Chat

#### urlId

Domyślnie Collab Chat generuje unikalny identyfikator dla każdej konwersacji na podstawie URL strony, ścieżki DOM do elementu i wybranego zakresu tekstu. Możesz to nadpisać własnym `urlId`.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Jest to przydatne, gdy struktura URL może się zmieniać, ale chcesz zachować te same rozmowy, lub gdy chcesz udostępniać adnotacje na wielu stronach.

#### topBarTarget

Kontroluje wyświetlanie paska górnego, który pokazuje liczbę użytkowników i liczbę dyskusji. Ustaw na `null`, aby całkowicie wyłączyć pasek górny, lub podaj element DOM, aby renderować go w określonym miejscu.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Wyłącz pasek górny
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Renderuj pasek górny w niestandardowej lokalizacji
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Włącz stylizację trybu ciemnego, gdy Twoja strona ma ciemne tło. To wykrywanie jest automatyczne, ale możesz chcieć je nadpisać.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Funkcja zwrotna (callback), która jest wywoływana zawsze, gdy zmienia się liczba komentarzy. Przydatne do aktualizowania elementów UI, takich jak odznaki lub tytuły stron.

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

### Dziedziczone opcje konfiguracji

Ponieważ Collab Chat rozszerza standardowy widżet komentarzy, możesz użyć dowolnej opcji konfiguracyjnej z podstawowego widżetu FastComments. Oto kilka często używanych opcji:

#### locale

Ustaw język interfejsu widżetu. FastComments obsługuje dziesiątki języków.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // hiszpański
});
[inline-code-end]

#### readonly

Ustaw wszystkie konwersacje jako tylko do odczytu. Użytkownicy mogą przeglądać istniejące adnotacje, ale nie mogą tworzyć nowych ani odpowiadać.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Zintegruj z systemem uwierzytelniania za pomocą Single Sign-On.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Konfiguracja SSO
    }
});
[inline-code-end]

Zobacz dokumentację SSO, aby uzyskać pełne informacje o opcjach uwierzytelniania.

#### maxReplyDepth

Kontroluj, jak głęboko mogą sięgać odpowiedzi. Domyślnie Collab Chat ustawia to na 0, co oznacza, że wszystkie komentarze są płaskie (bez zagnieżdżonych odpowiedzi). Możesz to zmienić, jeśli chcesz rozmów w formie wątków.

[inline-code-attrs-start title = "Przykład konfiguracji"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Pozwól na 3 poziomy zagnieżdżenia
});
[inline-code-end]

### Konfiguracja wewnętrzna

Te opcje są ustawiane automatycznie przez Collab Chat i nie powinny być nadpisywane:

Wartość `productId` jest automatycznie ustawiana na `3` dla Collab Chat. Rozszerzenie `floating-chat` jest automatycznie ładowane, aby zapewnić funkcjonalność okna czatu. Widżet automatycznie wykrywa urządzenia mobilne (ekrany poniżej 768px szerokości) i odpowiednio dostosowuje UI.

### Pełny przykład

Poniżej przykład pokazujący kilka opcji konfiguracyjnych razem:

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
        // Twoja konfiguracja SSO
    },
    maxReplyDepth: 1
});
[inline-code-end]

Pełną listę wszystkich dostępnych opcji konfiguracyjnych dziedziczonych z podstawowego widżetu znajdziesz w głównej dokumentacji konfiguracji FastComments.