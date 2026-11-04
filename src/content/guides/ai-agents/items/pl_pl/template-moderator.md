**Identyfikator szablonu:** `tos_enforcer`

Szablon Moderator to zalecany punkt wyjścia, jeśli Twoim celem jest zmniejszenie obciążenia ręczną moderacją. Przegląda nowe i oflagowane komentarze oraz stosuje zasady Twojej społeczności.

Prawie zawsze będziesz chciał **rozszerzyć wbudowany prompt** o konkretne przykłady tego, co Twoja strona dopuszcza, a czego nie. Wbudowana w platformę polityka eskalacji (ostrzeżenie przed banem, przeszukanie pamięci przed banem) jest już zawarta w promptcie systemowym otrzymywanym przez agenta, więc nie musisz jej powtarzać.

### Wyzwalacze

- **Opublikowanie nowego komentarza** (`COMMENT_ADD`) - agent analizuje każdy nowy komentarz.
- **Komentarz przekracza próg flag** (`COMMENT_FLAG_THRESHOLD`, domyślny próg: 3) - agent ponownie ocenia komentarz oflagowany przez innych użytkowników.

### Dozwolone narzędzia

- [`mark_comment_approved`](#tools-overview) - przydatne w tenantach z pre-moderacją, gdzie agent zwalnia czyste komentarze i ukrywa pozostałe.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Nie może publikować komentarzy, głosować, przypinać, blokować, przyznawać odznak ani wysyłać e-maili - prompt jest celowo wąski.

### Zalecane kroki przed uruchomieniem

- **Ustaw [Zasady społeczności](#community-guidelines).** Wystarczy kilka zdań spisanej polityki; agent stosuje ją przy każdym uruchomieniu.
- **Zabezpiecz `ban_user` za pomocą [zatwierdzania](#approval-workflow).** To jest domyślnie włączone w regionie UE (zobacz [Zgodność z art. 17 DSA UE](#eu-dsa-compliance)) i zalecane wszędzie.
- **Rozważ również zabezpieczenie `mark_comment_spam` zatwierdzaniem**, jeśli masz treści o małym wolumenie, ale wysokich stawkach.
- **Zabezpiecz `mark_comment_approved` zatwierdzaniem, jeśli korzystasz z pre-moderacji.** Zatwierdzenie złego komentarza pokazuje go czytelnikom; zabezpiecz to, dopóki agent nie zdobył zaufania w trybie dry-run.
- **Zaznacz "Uwzględnij czynnik zaufania komentującego, wiek konta, historię banów i ostatnie komentarze"** w [Opcjach kontekstu](#context-options). Model będzie ostrzegał znacznie mniej agresywnie, gdy widzi, że ktoś jest od dawna użytkownikiem działającym w dobrej wierze.

### Zalecane okno dry-run

Uruchom ten szablon w trybie [dry-run](#dry-run-mode) przez co najmniej tydzień na rzeczywistym ruchu przed przełączeniem na Włączony. Użyj [Testów (odtworzeń)](#test-runs-replays), aby również zapodglądać działanie na poprzednich 30 dniach.