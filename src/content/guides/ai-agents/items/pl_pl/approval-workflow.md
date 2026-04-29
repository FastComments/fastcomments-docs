Zatwierdzenie to wywołanie narzędzia umieszczone w kolejce, które wymaga zatwierdzenia lub odrzucenia przez człowieka, zanim platforma je wykona.

### Konfigurowanie zatwierdzeń

W formularzu edycji agenta sekcja **Approvals** wyświetla wszystkie narzędzia, które agent może wywołać (listę dozwolonych) i pozwala zaznaczyć te, które muszą być przejrzane przez człowieka. Niezaznaczone narzędzia wykonują się natychmiast. Zaznaczone narzędzia trafiają do kolejki.

Niedozwolone narzędzia są *bezwarunkowo odrzucane*, nie trafiają do kolejki — zatwierdzenia dotyczą tylko narzędzi, które w przeciwnym razie byłyby dozwolone.

### Co się dzieje, gdy wywołane zostanie działanie wymagające zatwierdzenia

1. Agent wybiera wywołanie narzędzia (np. `ban_user`) z argumentami, uzasadnieniem i oceną pewności.
2. Zamiast wykonać je od razu, platforma umieszcza zatwierdzenie w stanie `PENDING` w kolejce, z nazwą narzędzia, argumentami, uzasadnieniem, oceną pewności oraz migawką kontekstu opisującą wyzwalacz, który je spowodował.
3. Powiadomienia są wysyłane do recenzentów (zob. [Powiadomienia o zatwierdzeniach](#approval-notifications)).
4. Bieg agenta kończy się i zostaje zarejestrowany — działanie jest pokazane jako **Oczekuje na zatwierdzenie** w [Widoku szczegółów uruchomienia](#run-detail-view).

### Przegląd zatwierdzeń

Skrzynka zatwierdzeń pod adresem [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) wyświetla zatwierdzenia w stanie oczekującym, zatwierdzone, odrzucone i z nieudanym wykonaniem. Dla każdego:

- **Nazwa narzędzia i argumenty** - dokładnie to, co agent chce wykonać.
- **Uzasadnienie agenta** - uzasadnienie podane przez agenta.
- **Pewność** - samoocena pewności agenta, od 0.0 do 1.0.
- **Migawka kontekstu** - komentarz, strona i użytkownik, których dotyczy akcja.

Dwa przyciski: **Zatwierdź** i **Odrzuć**. Zatwierdź faktycznie wykonuje narzędzie; Odrzuć odrzuca (kasuje) żądanie.

### Stany zatwierdzeń

Zatwierdzenie przechodzi przez następujące stany:

| Stan | Znaczenie |
|---|---|
| `PENDING` | Oczekuje na decyzję człowieka. |
| `APPROVED` | Człowiek zatwierdził i działanie zostało wykonane. |
| `REJECTED` | Człowiek odrzucił. Działanie nie zostało wykonane. |
| `EXECUTION_FAILED` | Człowiek zatwierdził, ale wykonanie nie powiodło się (np. docelowy komentarz został już usunięty). |
| `EXECUTING` | Przejściowy: człowiek kliknął Zatwierdź i działanie jest w trakcie wykonywania. Używany do sekwencjonowania równoczesnych kliknięć Zatwierdź, aby narzędzie powodujące rzeczywiste skutki uboczne nigdy nie uruchomiło się dwukrotnie. |

Stan `EXECUTING` ma znaczenie, gdy dwóch recenzentów kliknie Zatwierdź jednocześnie — jeden „wygrywa”, a drugi widzi, że zatwierdzenie już się przesunęło.

### Co jest usuwane

Zatwierdzenia oczekujące pozostają w stanie oczekiwania do czasu podjęcia decyzji. Wygasają automatycznie po **90 dniach** od utworzenia. Zatwierdzenia w każdym innym stanie również są usuwane po 90 dniach w celu porządkowania przestrzeni dyskowej.

Pola zatwierdzenia „decided by” / „decided at” / „executed at” / „execution result” są wypełniane w miarę przechodzenia zatwierdzenia przez stany — wszystko widoczne w widoku szczegółów skrzynki zatwierdzeń.

### Integracja webhooków

Dwa zdarzenia webhook uruchamiane są w miarę zmiany zatwierdzeń:

- **`approval.requested`** - przy wstawieniu PENDING.
- **`approval.decided`** - przy przejściu do APPROVED, REJECTED lub EXECUTION_FAILED.

Oba są podpisane jak każdy inny webhook. Zobacz [Zdarzenia webhooków](#webhook-events) i [Treści webhooków](#webhook-payloads).

### Zwiększenie bezpieczeństwa: bramka znanych narzędzi

Zatwierdzenia odmawiają umieszczenia w kolejce nazwy narzędzia, która nie jest rozpoznawanym narzędziem agenta. Jest to kontrola obrony w głębokości: nawet jeśli przyszła ścieżka kodu przekaże do przepływu zatwierdzeń nazwę narzędzia pochodzącą z LLM, ten ciąg nigdy nie może pojawić się jako zatwierdzenie w kolejce. Zestaw znanych nazw narzędzi to ten sam zestaw wymieniony w [Referencji narzędzi](#tools-overview).

### Typowe wzorce bramkowania

- **Nowy agent moderacji** - wymagaj zatwierdzenia dla `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Obserwuj skrzynkę przez kilka tygodni, potem usuń wymóg zatwierdzenia dla narzędzi o niskim współczynniku błędów.
- **Długoterminowy agent moderacji** - zachowaj `ban_user` i wszelkie działania nieodwracalne (`deleteAllUsersComments`, `banIP`) pod stałą kontrolą.
- **Region UE** - `ban_user` jest wymuszony przez Article 17 niezależnie od tego, co zaznaczysz. Zobacz [Zgodność z artykułem 17 DSA UE](#eu-dsa-compliance).

### Czego zatwierdzenia **nie** robią

- Nie opóźniają innych wywołań narzędzi przez agenta. Bieg agenta może wywołać kilka narzędzi, a tylko te objęte bramką trafiają do kolejki — reszta wykonuje się normalnie.
- Nie cofną biegu agenta, jeśli człowiek odrzuci. Niezabezpieczona część biegu została już wykonana.