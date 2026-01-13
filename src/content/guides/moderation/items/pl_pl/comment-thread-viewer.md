---
Podczas moderowania i przeglądania wątków komentarzy pożądane jest, aby mieć możliwość bezpośredniego przejścia do wątku, by uzyskać kontekst podczas moderowania.

Oznacza to, że przepływ użytkownika zaczyna się na stronie Moderacja komentarzy, a następnie musiałby przejść od pojedynczego komentarza do strony zawierającej ten komentarz, poczekać na załadowanie tej strony, poczekać na załadowanie komentarzy, a następnie przewinąć do tego komentarza.

Jednak FastComments oferuje szybszy sposób. Na stronie Moderacja komentarzy, obok każdego komentarza, w prawym dolnym rogu znajduje się przycisk „Wyświetl komentarz”.

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

Jeżeli ten komentarz ma odpowiedzi, tekst przycisku zamiast tego pokaże liczbę odpowiedzi, ale kliknięcie wykonuje tę samą akcję.

Ten przycisk przeniesie cię do **Widoku wątku komentarzy**.

Widok wątku komentarzy to niewielka, szybko ładująca się aplikacja hostowana przez FastComments, która renderuje wątek komentarzy dla strony, na której znajduje się komentarz, i przewija do tego komentarza.

Pozwala to moderatorom szybko uzyskać potrzebny kontekst, bez konieczności oczekiwania na załadowanie innej strony.

---