---
An **AI Agent** to autonomiczny pracownik, przypisany do twojego FastComments tenant, który obserwuje zdarzenia w twojej społeczności i podejmuje działania w twoim imieniu.

Każdy agent ma trzy elementy, które kontrolujesz:

1. **Osobowość.** Początkowy prompt w postaci wolnego tekstu, który definiuje ton, rolę i styl podejmowania decyzji ("Jesteś serdecznym gospodarzem społeczności", "Egzekwujesz zasady społeczności, ale raczej ostrzegasz niż banisz" i tak dalej).
2. **Jedno lub więcej wyzwalaczy.** Lista zdarzeń, które budzą agenta — nowy komentarz, komentarz przekraczający próg głosów lub zgłoszeń, akcja moderatora, pierwszy komentarz użytkownika na stronie i inne. Pełna lista znajduje się w [Przegląd zdarzeń wyzwalających](#triggers-overview).
3. **Lista dozwolonych narzędzi.** Co agent może robić — dodać komentarz, zagłosować, przypiąć, zamknąć, oznaczyć jako spam, zbanować użytkownika, ostrzec przez DM, przyznać odznakę, wysłać e-mail, zapisać i przeszukać współdzieloną pamięć. Pełna lista znajduje się w [Przegląd dozwolonych wywołań narzędzi](#tools-overview).

Kiedy wyzwalacz się uruchomi, agent otrzymuje wiadomość kontekstową opisującą, co się stało (komentarz, strona, opcjonalny kontekst wątku/użytkownika/strony) i jest uruchamiany z jego promptem początkowym oraz twoimi wytycznymi społeczności. Następnie wywołuje narzędzia, aby działać, zapisując uzasadnienie i ocenę pewności przy każdym wywołaniu.

### Agenci działają asynchronicznie

Agenci **nigdy nie blokują akcji użytkownika, która ich uruchomiła**. Czytelnik publikuje komentarz, komentarz jest zapisany i wyświetlony w wątku, odpowiedź zostaje zwrócona, i dopiero *potem* agent na nim działa — natychmiast lub po skonfigurowanym opóźnieniu (zob. [Wyzwalacze opóźnione](#trigger-deferred-delay)). Nic, co robi agent, nie zwiększa opóźnienia widocznego dla użytkownika.

### Dlaczego ich używać

- **Moderuj na dużą skalę.** Oznaczaj oczywisty spam i blokuj powracających sprawców bez konieczności ciągłego monitorowania kolejki.
- **Witaj nowych komentujących.** Odpowiadaj pierwszorazowym komentującym w twoim stylu.
- **Wyeksponuj najlepsze treści.** Przypinaj merytoryczne komentarze najwyższego poziomu po przekroczeniu przez nie progu głosów.
- **Stosuj zasady konsekwentnie.** Stosuj ten sam tekst polityki wobec każdego komentarza na granicy zasad.
- **Podsumowuj długie wątki.** Publikuj neutralne podsumowania wielostronicowych debat.

### Co pozwala zachować kontrolę

- **Tryb Dry Run.** Każdy nowy agent uruchamia się w **Dry Run**: przetwarza wyzwalacze, uruchamia model i zapisuje, co by zrobił, ale nie podejmuje żadnych rzeczywistych działań. Zobacz [Tryb Dry-Run](#dry-run-mode).
- **Zatwierdzenia.** Dowolny podzbiór działań może wymagać zatwierdzenia przez człowieka. Zobacz [Procedura zatwierdzania](#approval-workflow).
- **Budżety na agenta i konto.** Twarde limity dzienne i miesięczne. Zobacz [Przegląd budżetów](#budgets-overview).
- **Lista dozwolonych narzędzi.** Niedozwolone narzędzia są usuwane z palety modelu — agent dosłownie nie może ich zażądać. Zobacz [Przegląd dozwolonych wywołań narzędzi](#tools-overview).
- **Pola audytu przy każdej akcji.** Model musi dołączyć uzasadnienie i ocenę pewności. Oba pojawiają się na osi czasu uruchomienia i przy każdym zatwierdzeniu. Zobacz [Widok szczegółów uruchomienia](#run-detail-view).
- **Artykuł 17 DSA UE.** W regionie UE w pełni zautomatyzowane blokady są zablokowane. Zobacz [Zgodność z artykułem 17 DSA UE](#eu-dsa-compliance).
- **Brak trenowania na twoich danych.** FastComments korzysta z dostawców, którzy nie trenują na twoich promptach ani komentarzach.

### Gdzie wpisują się obok moderacji ludzkiej

Agenci i moderatorzy ludzie korzystają z tej samej platformy komentarzy: agenci wykonują działania przez te same kanały (approve, spam, ban, badge, pin, lock, write) i te działania pojawiają się w tych samych [Dziennikach komentarzy](/guide-moderation.html#comment-logs), tej samej [Stronie moderacji](/guide-moderation.html#moderate-comments-page) i tych samych strumieniach powiadomień. Agenci i ludzie widzą pracę siebie nawzajem i mogą na nią reagować — działania moderatorów same w sobie są prawidłowymi wyzwalaczami agenta (zob. [Wyzwalacz: Komentarz oceniony przez moderatora](#trigger-moderator-reviewed) i podobne).

---