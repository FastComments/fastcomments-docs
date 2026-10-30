An **AI Agent** to autonomiczny pracownik, przypisany do Twojego najemcy FastComments, który obserwuje zdarzenia w Twojej społeczności i podejmuje działania w Twoim imieniu.

Każdy agent ma trzy elementy, które kontrolujesz:

1. **Osobowość.** Wolny tekst początkowego polecenia, który definiuje ton, rolę i styl podejmowania decyzji („Jesteś ciepłym powitaczem społeczności”, „Egzekwujesz zasady społeczności, ale skłaniasz się ku ostrzeżeniom zamiast banowania” i tak dalej).
2. **Jedno lub więcej wyzwalaczy.** Lista zdarzeń, które budzą agenta – nowy komentarz, komentarz przekraczający próg głosów lub flag, akcja moderatora, pierwszy komentarz użytkownika na stronie i inne. Pełna lista znajduje się w [Trigger Events Overview](#triggers-overview).
3. **Lista dozwolonych narzędzi.** Co agent może robić – publikować komentarz, głosować, przypinać, blokować, oznaczać jako spam, banować użytkownika, ostrzegać przez DM, przyznawać odznakę, wysyłać e‑mail, zapisywać i przeszukiwać współdzieloną pamięć. Pełna lista znajduje się w [Allowed Tool Calls Overview](#tools-overview).

Gdy wyzwalacz zostanie uruchomiony, agent otrzymuje wiadomość kontekstową opisującą, co się wydarzyło (komentarz, strona, opcjonalny kontekst wątku/użytkownika/strony) i jest poddany swojemu początkowemu poleceniu oraz wytycznym Twojej społeczności. Następnie wywołuje narzędzia, aby działać, rejestrując uzasadnienie i wynik pewności przy każdym wywołaniu.

### Agenci działają asynchronicznie

Agenci **nigdy nie blokują akcji użytkownika, która ich wywołała**. Czytelnik publikuje komentarz, komentarz jest zapisywany i wyświetlany w wątku, odpowiedź jest zwracana i dopiero *wtedy* agent działa na nim – natychmiast lub po skonfigurowanym opóźnieniu (zobacz [Deferred Triggers](#trigger-deferred-delay)). Nic, co robi agent, nie zwiększa opóźnienia w doświadczeniu użytkownika.

### Dlaczego ich używać

- **Moderowanie w skali.** Oznacz oczywisty spam i banuj powtarzających się sprawców bez konieczności ciągłego monitorowania kolejki.
- **Witamy nowych komentatorów.** Odpowiadaj pierwszym komentatorom w Twoim stylu.
- **Wydobywaj najlepszą treść.** Przypinaj istotne komentarze najwyższego poziomu, gdy przekroczą próg głosów.
- **Egzekwuj wytyczne konsekwentnie.** Stosuj ten sam tekst polityki do każdego komentarza granicznego.
- **Podsumowuj długie wątki.** Publikuj neutralne podsumowania wielostronicowych debat.

### Co daje Ci kontrolę

- **Tryb symulacji (Dry-run).** Każdy nowy agent jest dostarczany w trybie **Dry Run**: przetwarza wyzwalacze, uruchamia model i zapisuje, co *zrobiłby*, ale nie podejmuje rzeczywistych działań. Zobacz [Dry-Run Mode](#dry-run-mode).
- **Zatwierdzenia.** Dowolny podzbiór działań może być uzależniony od zatwierdzenia przez człowieka. Zobacz [Approval Workflow](#approval-workflow).
- **Budżety per-agent i per-konto.** Sztywne limity dzienne i miesięczne. Zobacz [Budgets Overview](#budgets-overview).
- **Lista dozwolonych narzędzi.** Niedozwolone narzędzia są usuwane z palety modelu – agent dosłownie nie może ich żądać. Zobacz [Allowed Tool Calls Overview](#tools-overview).
- **Pola audytu przy każdej akcji.** Model musi zawierać uzasadnienie i wynik pewności. Oba pojawiają się w osi czasu uruchomienia i przy każdym zatwierdzeniu. Zobacz [Run Detail View](#run-detail-view).
- **Artykuł 17 EU DSA.** W regionie UE w pełni zautomatyzowane bany są blokowane. Zobacz [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Brak uczenia na Twoich danych.** FastComments korzysta z dostawców, którzy nie trenują na Twoich poleceniach ani komentarzach.

### Gdzie pasują w zestawieniu z moderacją ludzką

Agenci i moderatorzy ludzcy korzystają z tej samej platformy komentarzy: agenci podejmują działania przez te same kanały (zatwierdzanie, spam, ban, odznaka, przypięcie, blokada, pisanie) i te działania pojawiają się w tych samych [Comment Logs](/guide-moderation.html#comment-logs), tej samej [Moderation Page](/guide-moderation.html#moderate-comments-page) oraz tych samych strumieniach powiadomień. Agenci i ludzie widzą nawzajem swoją pracę i mogą na siebie reagować – akcje moderatorów są same w sobie ważnymi wyzwalaczami dla agentów (zobacz [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) i podobne).

Jeśli potrzebujesz również ludzi w kolejce, FastComments oferuje [Human Moderation Services](/guide-moderation.html#human-moderation-services): nasi moderatorzy pracują razem z Twoimi agentami, w Twoim panelu, zgodnie z Twoimi wytycznymi.