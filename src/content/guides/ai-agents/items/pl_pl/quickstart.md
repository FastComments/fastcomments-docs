---
To jest pięciominutowa ścieżka od "mamy Agentów AI" do "agent odpowiada na ruch na żywo, zablokowany przez zatwierdzenia". Jeśli chcesz wersję szczegółową, każdy krok zawiera link do strony, która omawia go dogłębnie.

### 1. Otwórz stronę Agenci AI

Przejdź do [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) w swoim koncie. Za pierwszym razem zobaczysz albo:

- Stan pusty z przyciskami **Przeglądaj szablony** i **Rozpocznij od zera** (masz dostępne do utworzenia agentów), albo
- stronę upsell, jeśli Twój plan nie obejmuje agentów - zobacz [Plans and Eligibility](#plans-and-eligibility).

### 2. Wybierz szablon startowy

Kliknij **Przeglądaj szablony**. Wybierz jeden z:

- [Moderator](#template-moderator) - przegląda oznaczone lub nowe komentarze, ostrzega osoby po raz pierwszy, eskaluje do bana dopiero po ostrzeżeniu.
- [Welcome Greeter](#template-welcome-greeter) - odpowiada pierwszym komentującym.
- [Top Comment Pinner](#template-top-comment-pinner) - przypina merytoryczne komentarze, gdy przekroczą próg głosów.
- [Thread Summarizer](#template-thread-summarizer) - publikuje neutralne podsumowanie długich wątków.

Każdy szablon otwiera wstępnie wypełniony formularz edycji z już wybranym **Status: Tryb testowy**.

### 3. Przejrzyj i zapisz

W formularzu edycji wykonaj przynajmniej:

- **Nazwa wewnętrzna.** Krótki identyfikator używany w pulpicie administracyjnym.
- **Nazwa wyświetlana.** Co pojawia się publicznie, gdy agent opublikuje komentarz.
- **Początkowy prompt.** Edytuj prompt szablonu, aby dopasować go do Twojego stylu i konkretnych reguł.
- **Zatwierdzenia.** Zaznacz akcje, które powinny wymagać przeglądu przez człowieka, zanim zostaną wykonane. Zalecamy co najmniej `ban_user` dla każdego agenta działającego w stylu moderacji. Zobacz [Approval Workflow](#approval-workflow).

Kliknij **Zapisz agenta**.

### 4. Obserwuj w trybie testowym

Agent jest teraz aktywny w **Tryb testowy**. Będzie otrzymywał wyzwalacze, wywoływał model i zapisywał akcje na stronie [Run History](#run-history) - z odznaką **Tryb testowy** przy każdym wierszu - ale nie podejmuje prawdziwych działań. Odwiedź kilka szczegółów uruchomień (zobacz [Run Detail View](#run-detail-view)) i przyjrzyj się:

- Akcjom wybranym przez agenta.
- Uzasadnieniu i pewności dla każdej akcji.
- Pełnemu zapisowi rozmowy z LLM.

Jeśli agent podejmuje decyzje, z którymi się nie zgadzasz, edytuj początkowy prompt lub zaznacz więcej zatwierdzeń.

### 5. Uruchom test przeciwko przeszłym komentarzom

Na stronie listy agentów kliknij **Uruchom test** w wierszu agenta. Formularz ma jedno pole numeryczne **Dni** (1 do 90). Próbka i sztywny limit ocenianych komentarzy są pokazywane informacyjnie - są obliczane po stronie serwera, nie ustawiane przez użytkownika. Odtworzenie uruchamia analizę historycznych komentarzy bez podejmowania prawdziwych działań i raportuje, co agent **zrobiłby** w porównaniu z tym, co faktycznie się stało (czy komentarz później został zatwierdzony, oznaczony jako spam, usunięty itp.). Zobacz [Test Runs (Replays)](#test-runs-replays).

### 6. Przełącz na Włączony

Kiedy będziesz zadowolony z wyników w trybie testowym i odtworzeń, edytuj agenta i zmień **Status** na **Włączony**. Od tej pory trafiają prawdziwe akcje. Strona Historia uruchomień teraz pokazuje uruchomienia na żywo bez odznaki trybu testowego, a każda akcja, którą oznaczyłeś do zatwierdzenia, pojawia się w [approvals inbox](#approval-workflow).

### Co dalej

- Ustaw [Budżety](#budgets-overview) i [Alerty budżetowe](#budget-alerts).
- Skonfiguruj [Webhooki](#webhooks-overview), jeśli chcesz, aby systemy zewnętrzne reagowały na zdarzenia agenta.
- Dodaj [Zasady społeczności](#community-guidelines), aby decyzje agenta były zgodne z Twoją pisemną polityką.

---