---
Domyślnie agent uruchamia się **natychmiastowo** po wyzwoleniu wyzwalacza. Pole **Opóźnienie przed uruchomieniem** w formularzu edycji zmienia to: platforma umieszcza wyzwalacz w kolejce i uruchamia agenta w zaplanowanym czasie.

### Kiedy użyć opóźnienia

- **Wyzwalacze oparte na progu zgłoszeń (flag-threshold triggers)** - zgłoszenia często napływają partiami. Opóźnienie 10–30 minut pozwala sytuacji się ustabilizować, dzięki czemu agent działa na podstawie końcowego zliczenia zgłoszeń, a nie momentu ich nadejścia.
- **Wyzwalacze oparte na progu głosów (vote-threshold triggers)** - ta sama logika, szczególnie przy masowym obniżaniu ocen (downvote brigading).
- **Podsumowywanie wątku** - szablon [Thread Summarizer](#template-thread-summarizer) domyślnie ma opóźnienie 30 minut, więc podsumowuje rozmowę, która miała czas się rozwinąć, a nie wątek z dwoma odpowiedziami.
- **Okres ochłodzenia / ponowna ocena** - "24 godziny po zablokowaniu komentarza, rozważ, czy go odblokować."

### Configuration

- **Field**: Delay before running.
- **Range**: 0 to 2,592,000 seconds (30 days).
- **Units**: Seconds, minutes, hours, or days.

### Idempotence

Kolejka opóźnionych wyzwalaczy nie usuwa duplikatów. Dwa zgłoszenia napływające z 1-sekundowym odstępem do agenta z 30-minutowym opóźnieniem oba zaplanują uruchomienie za 30 minut, a agent uruchomi się **dwukrotnie**, za każdym razem operując na (w dużej mierze) tym samym kontekście. Jeśli chcesz semantyki maksymalnie jednego uruchomienia na okno czasowe, agent musi to wymusić — zwykle przez zapisanie [memory note](#tools-overview) podczas pierwszego uruchomienia i sprawdzenie jej przy kolejnych.

### Cost note

Opóźnione wyzwalacze są rejestrowane **zanim** zostaną uruchomione. Nagły napływ wyzwalaczy do agenta z dużym opóźnieniem może się gromadzić w kolejce bez zużywania tokenów; koszt ponoszony jest dopiero w momencie, gdy cron je wywoła. Użyj [Run History](#run-history) i [Drop Reasons](#drop-reasons), aby zobaczyć, jak często opóźnione wyzwalacze faktycznie się wykonują, a jak często są odrzucane podczas uruchomienia z powodów budżetowych.

### Replay does not honor delay

Funkcja [Test Runs (Replays)](#test-runs-replays) uruchamia agenta natychmiast na historycznych komentarzach — nie czeka na skonfigurowane opóźnienie. Traktuj to jako funkcję: replaye służą do podglądu tego, co agent **zrobiłby** w danym kontekście, a nie do odtwarzania rzeczywistego harmonogramu.
---