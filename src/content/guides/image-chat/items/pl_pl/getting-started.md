### Przypadki użycia

Image Chat sprawdza się doskonale przy opiniowaniu projektów, gdy zespoły muszą omawiać konkretne elementy w makietach lub zrzutach ekranu. Serwisy z recenzjami produktów mogą pozwolić klientom dyskutować o konkretnych cechach widocznych na zdjęciach produktów. Platformy edukacyjne mogą wykorzystywać go do omawiania diagramów, map lub obrazów naukowych. Galerie zdjęć mogą stać się interaktywne dzięki komentarzom przypisanym do określonych miejsc. Serwisy nieruchomości mogą pozwolić widzom dyskutować o konkretnych cechach widocznych na zdjęciach nieruchomości.

### Szybki start

Rozpoczęcie pracy z Image Chat jest proste. Potrzebujesz skryptu FastComments Image Chat, elementu obrazu lub kontenera z obrazem oraz obiektu konfiguracji z Twoim Tenant ID.

### Instalacja

Dodaj skrypt Image Chat do swojej strony:

[inline-code-attrs-start title = 'Ładowanie skryptu Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Podstawowa implementacja

Here's a minimal example:

[inline-code-attrs-start title = 'Podstawowa implementacja Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Twój obraz -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Załaduj skrypt Image Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Zainicjalizuj Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [panelu FastComments](https://fastcomments.com/auth/my-account).

### Jak to działa

Po zainicjowaniu użytkownicy mogą kliknąć dowolne miejsce na obrazie. Po kliknięciu w tym miejscu pojawia się wizualny kwadratowy znacznik, a okno czatu się otwiera. Inni użytkownicy widzą wszystkie znaczniki i mogą w nie klikać, aby wyświetlić dyskusję lub wziąć w niej udział. Wszystkie dyskusje synchronizują się w czasie rzeczywistym między odwiedzającymi.

Widżet wykorzystuje pozycjonowanie oparte na procentach, dzięki czemu znaczniki pozostają we właściwej pozycji nawet wtedy, gdy obraz zmienia rozmiar lub jest oglądany na różnych rozmiarach ekranu.

### Demo na żywo

Możesz zobaczyć Image Chat w akcji na naszej [stronie z demo na żywo](https://fastcomments.com/product/image-chat).

### Kolejne kroki

Teraz, gdy masz podstawy działające, możesz dostosować wygląd i zachowanie, korzystając z przewodnika Opcje konfiguracji. Zapoznaj się z przewodnikiem Projekt responsywny, aby zrozumieć, jak działa pozycjonowanie oparte na procentach. Dowiedz się o stylizacji i wsparciu trybu ciemnego w przewodniku Dostosowywanie. Dla zaawansowanych integracji przejrzyj Referencję API.

### Biblioteki frontendowe

Wszystkie biblioteki front-end FastComments (react, vue, angular itp.) zawierają Image Chat.