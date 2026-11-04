Domyślnie agent uruchamia się **natychmiast** po zadziałaniu jego wyzwalacza. Pole **Opóźnienie przed uruchomieniem** w formularzu edycji zmienia to zachowanie: platforma kolejkuję wyzwalacz i uruchamia agenta w zaplanowanym czasie.

### Kiedy używać opóźnienia

- **Wyzwalacze progowe flag** - flagi często przychodzą w seriach. Opóźnienie 10-30 minut pozwala sytuacji się ustabilizować, dzięki czemu agent działa na podstawie końcowej liczby flag, a nie stanu z chwili przybycia.
- **Wyzwalacze progowe głosów** - ta sama logika, szczególnie w przypadku brigadingu downvote'ów.
- **Podsumowywanie wątków** - [szablon Thread Summarizer](#template-thread-summarizer) domyślnie używa 30-minutowego opóźnienia, aby podsumować rozmowę, która miała czas się rozwinąć, a nie wątek z dwiema odpowiedziami.
- **Okres wyciszenia / ponowna ocena** - "24 godziny po zablokowaniu komentarza, rozważ, czy go odblokować."

### Konfiguracja

- **Pole**: Opóźnienie przed uruchomieniem.
- **Zakres**: od 0 do 2 592 000 sekund (30 dni).
- **Jednostki**: sekundy, minuty, godziny lub dni.

### Idempotencja

Kolejka odroczona nie deduplikuje wyzwalaczy. Dwie flagi przychodzące w odstępie 1 sekundy przy agencie z 30-minutowym opóźnieniem zaplanują oba uruchomienia 30 minut później, a agent uruchomi się **dwa razy**, oba razy na (w większości) tym samym kontekście. Jeśli chcesz semantyki co najwyżej jednego uruchomienia na okno czasowe, agent musi to wymusić - zazwyczaj zapisując [notatkę w pamięci](#tools-overview) przy pierwszym uruchomieniu i sprawdzając ją przy kolejnych.

### Uwaga o kosztach

Odroczone wyzwalacze są rejestrowane **przed** ich uruchomieniem. Seria wyzwalaczy na agencie z dużym opóźnieniem może nagromadzić się w kolejce bez zużycia tokenów; koszt jest ponoszony dopiero, gdy cron je wysyła. Użyj [Historii uruchomień](#run-history) i [Powodów odrzucenia](#drop-reasons), aby zobaczyć, jak często odroczone wyzwalacze faktycznie się wykonują, a jak często są odrzucane w czasie uruchomienia z powodu limitu budżetu.

### Replay nie uwzględnia opóźnienia

Funkcja [Test Runs (Replays)](#test-runs-replays) uruchamia agenta natychmiast na historycznych komentarzach - nie czeka na skonfigurowane opóźnienie. Traktuj to jako zaletę: replaye służą do podglądu, co agent **zrobiłby** w danym kontekście, a nie do odtwarzania harmonogramu w czasie rzeczywistym.