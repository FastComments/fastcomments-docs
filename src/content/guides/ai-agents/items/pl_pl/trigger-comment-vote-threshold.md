Wyzwalane, gdy łączna liczba głosów komentarza osiągnie skonfigurowany próg. Łączna liczba głosów to `votesUp - votesDown`.

### Wymagana konfiguracja

- **Vote threshold** - liczba całkowita >= 1. Wyzwalacz uruchamia się przy głosie, który sprawia, że łączna liczba głosów wynosi dokładnie tę wartość.

Jeśli próg wynosi 10 i komentarz przechodzi z 9 do 10 łącznych głosów, wyzwalacz uruchamia się raz. Jeśli następny głos podniesie wynik z 10 na 11, wyzwalacz **nie** uruchamia się ponownie - nie wyzwala się przy każdym kolejnym głosie powyżej progu.

### Kontekst, który otrzymuje agent

- Komentarz wraz z bieżącymi licznikami głosów.
- **vote direction** (`up` or `down`) głosu, który spowodował przekroczenie progu.
- Opcjonalna historia wątku / użytkownika / kontekst strony zgodnie z konfiguracją.

### Warto zauważyć

- Komentarz, który osiąga 10, spada do 9, a następnie ponownie rośnie do 10, uruchomi wyzwalacz dwukrotnie. Nie ma stanu "fired once" dla pojedynczego komentarza - jeśli potrzebujesz takiej semantyki, spraw, by agent zapisał [memory note](#tools-overview) przy pierwszym uruchomieniu i sprawdzał ją przy kolejnych uruchomieniach.
- Próg zawsze odnosi się do **net** liczby głosów, nie do surowych upvotes. Komentarz z 12 up i 2 down ma net 10 i uruchamia wyzwalacz; taki z 10 up i 0 down również go uruchamia.
- Możliwe są przekroczenia spowodowane wyłącznie głosem przeciw - komentarz przechodzący z 11 na 10 z powodu down-vote również uruchamia wyzwalacz. Parametr `voteDirection` w kontekście informuje agenta, z którego kierunku nastąpiło przekroczenie progu.

### Typowe zastosowania

- **Pinning** - the [Top Comment Pinner template](#template-top-comment-pinner) jest zbudowany wokół tego wyzwalacza.
- **Promotion / featured comment workflows** - wyemituj zdarzenie przez [Webhooks](#webhooks-overview), aby zewnętrzny system mógł promować komentarz w innych miejscach na Twojej stronie.
- **Engagement tracking** - zapisz pamięć (memory note) o użytkowniku, który napisał komentarz, aby inni agenci wiedzieli, że stworzył popularną treść.

### Dostosowywanie

Odpowiedni próg zależy od społeczności. Obserwuj [Run History](#run-history) przez kilka dni przy niskim progu (5), żeby zobaczyć, jak często się uruchamia. Podnoś próg, aż częstotliwość uruchomień będzie odpowiadać rytmowi, który naprawdę chcesz.