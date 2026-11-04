### Przypadki użycia

Image Chat świetnie sprawdza się przy opiniach na temat projektów, gdzie zespoły muszą omawiać konkretne elementy w makietach lub zrzutach ekranu. Strony z recenzjami produktów mogą pozwolić klientom omawiać konkretne funkcje widoczne na zdjęciach produktów. Platformy edukacyjne mogą go używać do omawiania diagramów, map lub obrazów naukowych. Galerie zdjęć mogą stać się interaktywne dzięki komentarzom przypisanym do konkretnych lokalizacji. Strony nieruchomości mogą pozwolić oglądającym omawiać konkretne cechy widoczne na zdjęciach nieruchomości.

### Szybki start

Rozpoczęcie pracy z Image Chat jest proste. Potrzebujesz skryptu FastComments Image Chat, elementu obrazu lub kontenera z obrazem oraz obiektu konfiguracyjnego z Twoim Tenant ID.

### Instalacja

Dodaj skrypt Image Chat do swojej strony:

[inline-code-attrs-start title = 'Ładowanie skryptu Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Podstawowa implementacja

Oto minimalny przykład:

[inline-code-attrs-start title = 'Podstawowa implementacja Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Your image -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Load the Image Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Initialize Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Zamień `'demo'` na swój rzeczywisty FastComments Tenant ID, jeśli jeszcze tego nie zrobiłeś — znajdziesz go w swoim [panelu FastComments](https://fastcomments.com/auth/my-account).

### Jak to działa

Po inicjalizacji użytkownicy mogą klikać w dowolne miejsce na obrazie. Po kliknięciu w tej lokalizacji pojawia się wizualny znacznik w kształcie kwadratu i otwiera się okno czatu. Inni użytkownicy mogą widzieć wszystkie znaczniki i klikać je, aby wyświetlić lub dołączyć do tych dyskusji. Wszystkie dyskusje synchronizują się w czasie rzeczywistym u wszystkich odwiedzających.

Widżet używa pozycjonowania opartego na procentach, więc znaczniki pozostają we właściwym miejscu nawet gdy obraz zmienia rozmiar lub jest oglądany na ekranach o różnych rozmiarach.

### Demo na żywo

Możesz zobaczyć Image Chat w akcji na naszej [stronie demo na żywo](https://fastcomments.com/product/image-chat).

### Następne kroki

Teraz, gdy masz działające podstawy, możesz dostosować wygląd i zachowanie w przewodniku Opcje konfiguracji. Zapoznaj się z przewodnikiem Projektowanie responsywne, aby zrozumieć, jak działa pozycjonowanie oparte na procentach. Dowiedz się o stylowaniu i obsłudze trybu ciemnego w przewodniku Dostosowywanie. W przypadku zaawansowanych integracji zapoznaj się z dokumentacją API.

### Biblioteki frontendowe

Wszystkie biblioteki frontendowe FastComments (react, vue, angular itd.) obsługują Image Chat.