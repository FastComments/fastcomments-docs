### Overview

FastComments Image Chat rozszerza standardowy widżet komentarzy FastComments, dzięki czemu dziedziczy wszystkie opcje konfiguracyjne podstawowego widżetu, jednocześnie dodając kilka ustawień specyficznych dla adnotacji obrazów.

### Required Configuration

#### tenantId

Wymagane jest Twoje ID najemcy (FastComments Tenant ID). Możesz je znaleźć w swoim [panelu FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specific Options

#### urlId

Domyślnie Image Chat generuje unikalny identyfikator dla każdej rozmowy na podstawie URL strony, źródła obrazu oraz współrzędnych X/Y. Możesz go nadpisać za pomocą niestandardowego `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Jest to przydatne, gdy struktura Twoich URL może się zmieniać, ale chcesz zachować te same rozmowy, lub gdy chcesz udostępniać adnotacje na wielu stronach.

#### chatSquarePercentage

Steruje rozmiarem klikalnych znaczników czatu jako procent szerokości obrazu. Domyślnie wynosi 5%, co oznacza, że każdy znacznik ma szerokość równą 5% szerokości obrazu.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Większe, bardziej widoczne znaczniki
});
```

Mniejsze wartości tworzą mniej inwazyjne znaczniki, które lepiej sprawdzają się przy szczegółowych obrazach. Większe wartości ułatwiają widoczność i klikanie znaczników na zatłoczonych obrazach lub dla użytkowników na urządzeniach mobilnych.

#### hasDarkBackground

Włącz stylizację w trybie ciemnym, gdy Twoja strona ma ciemne tło.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Funkcja zwrotna wywoływana za każdym razem, gdy zmienia się liczba komentarzy. Przydatne do aktualizowania elementów UI, takich jak odznaki lub tytuły stron.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Inherited Configuration Options

Ponieważ Image Chat rozszerza standardowy widżet komentarzy, możesz używać dowolnej opcji konfiguracyjnej z podstawowego widżetu FastComments. Oto niektóre często używane opcje:

#### locale

Ustaw język interfejsu widżetu. FastComments obsługuje dziesiątki języków.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Hiszpański
});
```

#### readonly

Ustaw wszystkie rozmowy jako tylko do odczytu. Użytkownicy mogą przeglądać istniejące znaczniki i dyskusje, ale nie mogą tworzyć nowych ani odpowiadać.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Zintegruj się z systemem uwierzytelniania za pomocą Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Konfiguracja SSO
    }
});
```

Zobacz dokumentację SSO, aby uzyskać pełne informacje o opcjach uwierzytelniania.

#### maxReplyDepth

Kontroluje, jak głęboko mogą sięgać odpowiedzi (liczba poziomów zagnieżdżeń). Domyślnie Image Chat ustawia tę wartość na 0, co oznacza, że wszystkie komentarze są płaskie (bez zagnieżdżonych odpowiedzi). Możesz to zmienić, jeśli chcesz wątki konwersacji.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Pozwól na 3 poziomy zagnieżdżenia
});
```

### Internal Configuration

Te opcje są automatycznie ustawiane przez Image Chat i nie powinny być nadpisywane:

The `productId` is automatically set to `2` for Image Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly with fullscreen chat windows.

### Target Element Flexibility

Pierwszy parametr funkcji `FastCommentsImageChat` może być bezpośrednio elementem `<img>` lub elementem kontenera z obrazem wewnątrz:

```javascript
// Bezpośredni element obrazu
FastCommentsImageChat(document.getElementById('my-image'), config);

// Kontener z obrazem wewnątrz
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Widżet automatycznie znajdzie obraz, jeśli przekażesz element kontenera.

### Complete Example

Oto przykład pokazujący kilka opcji konfiguracyjnych razem:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Twoja konfiguracja SSO
    },
    maxReplyDepth: 1
});
```

Pełną listę wszystkich dostępnych opcji konfiguracyjnych odziedziczonych po podstawowym widżecie znajdziesz w głównej dokumentacji konfiguracji FastComments.