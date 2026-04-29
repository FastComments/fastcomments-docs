The Ban tool is the most consequential action an agent can call. It bans a user from your community, with a fixed duration and a few options.

### Co robi

Agent wybiera jeden z sześciu okresów:

- Jedna godzina
- Jeden dzień
- Jeden tydzień
- Jeden miesiąc
- Sześć miesięcy
- Jeden rok

Agent wybiera także między **widocznym zbanowaniem** (użytkownik widzi wyraźny komunikat o zbanowaniu i może się odwołać) a **shadow ban** (użytkownik może dalej publikować, ale jego treści są ukryte przed innymi użytkownikami). Instrukcje platformy nakazują agentowi preferować widoczne bany w przypadku pierwszych lub granicznych przypadków, a shadow bany wobec wyraźnie złośliwych powtarzających się naruszycieli.

### Dwie destrukcyjne podopcje

Dwie dodatkowe opcje są domyślnie **ukryte przed agentem**. Aby włączyć którąkolwiek, zaznacz odpowiednie pole wyboru w sekcji **Opcje bana** na formularzu edycji agenta:

- **Allow deleting all of the user's comments.** Po włączeniu agent może również zdecydować o usunięciu wszystkich komentarzy, które zbanowany użytkownik kiedykolwiek opublikował w Twoim tenant. Zarezerwuj to dla wyraźnego spamu, doxxingu lub skoordynowanego nadużycia, gdzie istniejące treści nie mają wartości. **Destrukcyjne i nieodwracalne.**
- **Allow banning by IP.** Po włączeniu agent może również zbanować adres IP, z którego opublikowano komentarz. Przydatne przeciwko omijaniu bana przez konta alternatywne. **Unikaj dla współdzielonych adresów IP** (firmowe, szkolne, operatorzy mobilni) - niewinni użytkownicy w tej samej sieci zostaną zablokowani.

Platforma dodatkowo narzuca te ograniczenia po stronie serwera: nawet jeśli agent zbuntuje się i spróbuje wywołać opcję, żądanie zostanie odrzucone, chyba że włączyłeś taką opcję.

### Polityka eskalacji

Zanim zbanować, platforma instruuje agenta, aby:

1. Przeszukać [pamięć agenta](#agent-memory-system) w poszukiwaniu wcześniejszych ostrzeżeń lub notatek dotyczących użytkownika.
2. W pierwszej kolejności woleć [ostrzeżenie](#tool-warn-user) użytkownika niż jego zbanowanie za pierwsze przewinienia.
3. Pominąć krok ostrzeżenia tylko w oczywistych, rażących przypadkach (treści nielegalne, doxxing, skoordynowany spam) — i wyjaśnić powód w uzasadnieniu.

Ta polityka znajduje się w instrukcjach agenta, a nie jest twardą regułą po stronie serwera, dlatego **mocno zaleca się uzależnienie banów od zatwierdzenia**.

### Region UE: wymagana akceptacja przez człowieka

W regionie UE to narzędzie jest **włączone z obowiązkiem zatwierdzenia** na mocy Artykułu 17 Rozporządzenia o usługach cyfrowych. Każdy ban od dowolnego agenta na tenant z regionu UE trafia do [skrzynki zatwierdzeń](#approval-workflow) do przeglądu przez człowieka. Zobacz [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Rekomendacje

- Uzależnij od zatwierdzenia we wszystkich miejscach przynajmniej przez pierwszy miesiąc.
- Zawsze uzależniaj opcję **delete-all-comments** jeśli ją włączysz - jest nieodwracalna.
- Rozważ uzależnienie opcji **IP ban** nawet po zdobyciu zaufania agenta - koszt zbanowania IP w sieci współdzielonej nie pojawia się w historii działań agenta.

### Zobacz także

- [Banning Users](/guide-moderation.html#banning-users) i [Banning Users With Wildcards](/guide-moderation.html#banning-users-wildcards) w przewodniku moderacji, aby dowiedzieć się, jak bany działają na całej platformie.
- [Ostrzeż użytkownika](#tool-warn-user) - łagodniejszy krok eskalacji.