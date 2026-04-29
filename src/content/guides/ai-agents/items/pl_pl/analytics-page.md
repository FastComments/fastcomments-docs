Analytics to pulpit nawigacyjny obejmujący wielu agentów. Dostępny ze strony Agentów AI przez kartę **Analiza** (na poziomie całego konta) lub per‑agent przez przycisk **Analiza** w wierszu każdego agenta.

### Filtr

Rozwijane menu u góry - **Wszyscy agenci** albo konkretny agent. Reszta strony dostosowuje się odpowiednio do zakresu.

### Wykorzystanie budżetu

Cztery paski postępu pokazujące wydatki w bieżącym okresie względem limitu:

- **Agent dziś** (gdy filtrowane do konkretnego agenta) - dzienny limit agenta.
- **Agent w tym miesiącu** - miesięczny limit agenta.
- **Konto dziś** - dzienny limit konta.
- **Konto w tym miesiącu** - miesięczny limit konta.

Gdy limit nie jest ustawiony, pasek pokazuje "(brak ustawionego limitu)" i wyświetla surowe wydatki.

### Dzienny koszt (ostatnie 30 dni)

Tabela kosztów na dzień w walucie Twojego konta dla wybranego zakresu. Przydatne do wychwytywania:

- **Nagłych skoków kosztów** - zwykle wynik pętli bez kontroli lub wirusowego komentarza, który uruchamia reguły.
- **Dryftu kosztów** - stopniowy wzrost kosztów dziennych wraz z rozwojem społeczności.

### Wykonane akcje

Podział typów akcji w bieżącym miesiącu - "Napisano komentarz: 47", "Oznaczono komentarz jako spam: 12" itd. Przydatne do sprawdzenia, czy agent działa zgodnie z oczekiwaniami.

### Pominięte wyzwalacze (ten miesiąc)

Liczby pogrupowane według [przyczyny odrzucenia](#drop-reasons):

- Przekroczenie dziennego limitu agenta / miesięcznego limitu agenta / dziennego limitu konta / miesięcznego limitu konta.
- Ograniczone przez limit szybkości.
- Wyczerpana współbieżność.

Jeśli widzisz tu odrzucenia, Twój agent trafia na limit budżetu lub limit zapytań i pomija wyzwalacze, na których w innym przypadku by działał. Zobacz [Przyczyny odrzuceń](#drop-reasons).

### Tryb testowy vs na żywo (ten miesiąc)

- **Uruchomienia włączone** - liczba uruchomień, które podjęły rzeczywiste działania w tym miesiącu.
- **Uruchomienia w trybie testowym** - liczba uruchomień w trybie dry-run w tym miesiącu.

Przydatny sygnał do strojenia: całkowicie nowy agent, który nie został jeszcze ustawiony jako Włączony, pokaże tylko uruchomienia w trybie testowym. Agent w stanie Włączony z zerowymi wartościami w tej sekcji jest nieaktywny — albo nie jest wyzwalany, albo jest poza zakresem, albo jego wyzwalacze są nieprawidłowo skonfigurowane.

### Najdrożsi agenci według miesięcznego kosztu

Gdy filtr to **Wszyscy agenci**, strona wyświetla agentów posortowanych według kosztu od początku miesiąca. Wyszukanie najbardziej kosztownego agenta to pierwszy krok w optymalizacji kosztów — zwykle rozwiązaniem jest „zacieśnić jego [opcje kontekstu](#context-options)” lub „obniżyć jego [limit budżetu](#budgets-overview)”.

### Agenci na lub blisko limitu

Szczegółowy podział agentów, których wydatki są na lub blisko ich indywidualnych limitów w bieżącym okresie:

- **blisko limitu** - ponad konfigurowalny procent limitu.
- **powyżej limitu** - faktycznie ograniczony, z {count} pominiętymi wyzwalaczami w tym okresie.

Kliknij agenta w tej tabeli, aby podnieść limit, zawęzić zakres lub wstrzymać go.

### Podsumowanie konta

Gdy filtr to **Wszyscy agenci**:

- **Wyzwalacze dziś** - liczba.
- **Wyzwalacze w tym miesiącu** - liczba.
- Dla każdego: przyrostek `dropped` pokazujący, ile zostało pominiętych.

### Waluta

Wszystkie wartości pieniężne wyświetlane są w walucie Twojego konta.

### Czego ta strona nie robi

- Nie pokazuje **szczegółowego rozbicia kosztów dla każdej akcji** - te informacje są w [Widoku szczegółów uruchomienia](#run-detail-view).
- Nie pokazuje **transkrypcji** ani **odpowiedzi LLM**.
- Nie pozwala na działania na agentach — edycja, wstrzymywanie, usuwanie odbywają się z listy agentów / strony edycji.