Narzędzie Ban to najbardziej doniosła akcja, jaką agent może wywołać. Blokuje użytkownika w Twojej społeczności na określony czas, z kilkoma opcjami.

### Co robi

Agent wybiera jeden z sześciu okresów:

- Jedna godzina
- Jeden dzień
- Jeden tydzień
- Jeden miesiąc
- Sześć miesięcy
- Jeden rok

Wybiera też między **widocznym banem** (użytkownik widzi wyraźny komunikat o banie i może się odwołać) a **shadow banem** (użytkownik może dalej publikować, ale jego treści są ukryte przed innymi użytkownikami). Instrukcje platformy nakazują agentowi preferować widoczne bany w przypadkach pierwszego razu lub granicznych, a shadow bany dla ewidentnie złośliwych recydywistów.

### Dwie destrukcyjne podopcje

Dwie dodatkowe opcje są **domyślnie ukryte przed agentem**. Aby którąkolwiek włączyć, zaznacz odpowiednie pole wyboru w sekcji **Ban options** na formularzu edycji agenta:

- **Allow deleting all of the user's comments.** Po włączeniu agent może zdecydować o usunięciu każdego komentarza, jaki zbanowany użytkownik kiedykolwiek opublikował w Twojej tenancie. Zarezerwuj dla ewidentnego spamu, doxxingu lub skoordynowanych nadużyć, gdzie istniejąca treść nie ma wartości. **Destrukcyjne i nieodwracalne.**
- **Allow banning by IP.** Po włączeniu agent może zdecydować o zbanowaniu również IP, z którego opublikowano komentarz. Przydatne przeciw omijaniu bana przez konta alternatywne. **Unikaj dla współdzielonych IP** (firmowych, szkolnych, operatorów mobilnych) - niewinni użytkownicy w tej samej sieci zostaną zablokowani.

Platforma dodatkowo ogranicza te opcje po stronie serwera: nawet jeśli agent wymknie się spod kontroli i spróbuje wywołać opcję, żądanie zostanie odrzucone, chyba że ją włączyłeś.

### Polityka eskalacji

Przed banem platforma nakazuje agentowi:

1. Wyszukać w [pamięci agenta](#agent-memory-system) wcześniejsze ostrzeżenia lub notatki o użytkowniku.
2. Preferować [ostrzeżenie](#tool-warn-user) użytkownika zamiast bana przy pierwszych przewinieniach.
3. Pominąć krok ostrzeżenia tylko w rażąco poważnych przypadkach (treści nielegalne, doxxing, skoordynowany spam) - i wyjaśnić dlaczego w swojej uzasadnieniu.

Ta polityka znajduje się w instrukcjach agenta, a nie jest twardą regułą serwerową, dlatego **ograniczanie banów zatwierdzeniem jest zdecydowanie zalecane**.

### Region UE: wymagane zatwierdzenie przez człowieka

W regionie UE to narzędzie jest **na stałe zablokowane za zatwierdzeniem** na mocy art. 17 Aktu o usługach cyfrowych. Każdy ban od dowolnego agenta na tenancie w regionie UE trafia do [skrzynki zatwierdzeń](#approval-workflow) do weryfikacji przez człowieka. Zobacz [Zgodność z EU DSA Article 17](#eu-dsa-compliance).

### Zalecenia

- Włącz zatwierdzanie wszędzie przez co najmniej pierwszy miesiąc.
- Zawsze ograniczaj zatwierdzeniem opcję **delete-all-comments**, jeśli ją włączysz - jest nieodwracalna.
- Rozważ ograniczenie zatwierdzeniem opcji **IP ban** nawet po tym, jak agent zdobędzie zaufanie - koszt IP bana w sieci współdzielonej nie uwidacznia się w historii uruchomień agenta.

### Zobacz także

- [Banning Users](/guide-moderation.html#banning-users) i [Banning Users With Wildcards](/guide-moderation.html#banning-users-wildcards) w przewodniku moderacji, aby dowiedzieć się, jak działają bany na poziomie całej platformy.
- [Warn user](#tool-warn-user) - łagodniejszy krok eskalacji.