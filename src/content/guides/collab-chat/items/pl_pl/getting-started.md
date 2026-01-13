### Szybki start

Rozpoczęcie pracy z Collab Chat jest proste. Potrzebujesz skryptu FastComments Collab Chat, elementu HTML zawierającego tekst, który chcesz komentować, oraz obiektu konfiguracyjnego z Twoim Tenant ID.

### Instalacja

Dodaj skrypt Collab Chat do swojej strony:

[inline-code-attrs-start title = 'Ładowanie skryptu Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Podstawowa implementacja

Oto minimalny przykład:

[inline-code-attrs-start title = 'Podstawowa implementacja Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Kontener zawartości -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Załaduj skrypt Collab Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Zainicjalizuj Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Zastąp `'demo'` swoim rzeczywistym FastComments Tenant ID, jeśli jeszcze tego nie zrobiłeś — znajdziesz go w swoim [panelu FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Jak to działa

Po zainicjowaniu użytkownicy mogą zaznaczać dowolny tekst wewnątrz docelowego elementu. Po krótkim opóźnieniu (3,5 sekundy na komputerach) pojawia się podpowiedź umożliwiająca rozpoczęcie dyskusji. Gdy dyskusja zostanie utworzona, na tekście pojawia się wizualne podświetlenie. Inni użytkownicy mogą najedzieć kursorem lub kliknąć podświetlenie, aby wyświetlić i uczestniczyć w dyskusji. Wszystkie dyskusje synchronizują się w czasie rzeczywistym między wszystkimi odwiedzającymi.

### Demonstacja na żywo

Możesz zobaczyć Collab Chat w akcji na naszej [stronie demonstracyjnej na żywo](https://fastcomments.com/product/collab-chat).

### Kolejne kroki

Skoro masz już podstawy działające, możesz dostosować wygląd i zachowanie w przewodniku Opcje konfiguracji. Zapoznaj się z przewodnikiem Zachowanie zaznaczania tekstu, aby zrozumieć, jak działa zaznaczanie tekstu. Dowiedz się o stylowaniu i wsparciu trybu ciemnego w przewodniku Personalizacja. Dla zaawansowanych integracji zapoznaj się z Dokumentacją API.

### Biblioteki frontendowe

Wszystkie biblioteki frontendowe FastComments (react, vue, angular, itp.) zawierają Collab Chat.

---