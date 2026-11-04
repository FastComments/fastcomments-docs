### Przegląd

FastComments Image Chat rozszerza standardowy widget komentarzy FastComments, więc dziedziczy wszystkie opcje konfiguracji z bazowego widgetu, dodając jednocześnie kilka opcji specyficznych dla adnotacji obrazów.

### Wymagana konfiguracja

#### tenantId

Wymagany jest Twój FastComments Tenant ID. Możesz go znaleźć w swoim [panelu FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Opcje specyficzne dla Image Chat

#### urlId

Domyślnie Image Chat generuje unikalny identyfikator dla każdej rozmowy na podstawie adresu URL strony, źródła obrazu oraz współrzędnych X/Y. Możesz to nadpisać własnym `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Jest to przydatne, gdy struktura Twoich adresów URL może się zmienić, ale chcesz zachować te same rozmowy, lub gdy chcesz udostępniać adnotacje na wielu stronach.

#### chatSquarePercentage

Kontroluje rozmiar klikalnych znaczników czatu jako procent szerokości obrazu. Wartość domyślna to 5%, co oznacza, że każdy znacznik ma 5% szerokości obrazu.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Większe, bardziej widoczne znaczniki
});
```

Mniejsze wartości tworzą mniej nachalne znaczniki, które lepiej sprawdzają się na szczegółowych obrazach. Większe wartości ułatwiają widzenie i klikanie znaczników na zatłoczonych obrazach lub dla użytkowników urządzeń mobilnych.

#### hasDarkBackground

Włącz stylizację trybu ciemnego, gdy Twoja strona ma ciemne tło.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Funkcja zwrotna wywoływana przy każdej zmianie liczby komentarzy. Jest to przydatne do aktualizacji elementów interfejsu, takich jak plakietki czy tytuły stron.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Odziedziczone opcje konfiguracji

Ponieważ Image Chat rozszerza standardowy widget komentarzy, możesz użyć dowolnej opcji konfiguracji z bazowego widgetu FastComments. Oto niektóre z najczęściej używanych opcji:

#### locale

Ustawia język interfejsu widgetu. FastComments obsługuje dziesiątki języków.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Hiszpański
});
```

#### readonly

Ustawia wszystkie rozmowy jako tylko do odczytu. Użytkownicy mogą przeglądać istniejące znaczniki i dyskusje, ale nie mogą tworzyć nowych ani odpowiadać.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso i simpleSSO

Integracja z Twoim systemem uwierzytelniania przy użyciu Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Konfiguracja SSO
    }
});
```

Zobacz dokumentację SSO, aby uzyskać pełne informacje na temat opcji uwierzytelniania.

#### maxReplyDepth

Kontroluje, na ile poziomów w głąb mogą schodzić odpowiedzi. Domyślnie Image Chat ustawia tę wartość na 0, co oznacza, że wszystkie komentarze są płaskie (bez zagnieżdżonych odpowiedzi). Możesz to zmienić, jeśli chcesz mieć wątkowe rozmowy.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Pozwól na 3 poziomy zagnieżdżenia
});
```

### Konfiguracja wewnętrzna

Te opcje są ustawiane automatycznie przez Image Chat i nie powinny być nadpisywane:

`productId` jest automatycznie ustawiane na `2` dla Image Chat. Rozszerzenie `floating-chat` jest automatycznie ładowane, aby zapewnić funkcjonalność okna czatu. Widget automatycznie wykrywa urządzenia mobilne (ekrany o szerokości poniżej 768px) i odpowiednio dostosowuje interfejs, używając pełnoekranowych okien czatu.

### Elastyczność elementu docelowego

Pierwszy parametr `FastCommentsImageChat` może być bezpośrednio elementem `<img>` lub elementem kontenera zawierającym obraz:

```javascript
// Bezpośrednio element obrazu
FastCommentsImageChat(document.getElementById('my-image'), config);

// Kontener z obrazem w środku
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Widget automatycznie znajdzie obraz, jeśli przekażesz element kontenera.

### Kompletny przykład

Oto przykład pokazujący kilka opcji konfiguracji razem:

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

Aby uzyskać pełną listę wszystkich dostępnych opcji konfiguracji odziedziczonych z bazowego widgetu, zobacz główną dokumentację konfiguracji FastComments.