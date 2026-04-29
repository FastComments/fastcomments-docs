**ID szablonu:** `tos_enforcer`

Szablon Moderatora jest zalecanym punktem wyjścia, jeśli Twoim celem jest zmniejszenie nakładu pracy moderacji ręcznej. Przegląda nowe i oznaczone komentarze oraz stosuje Twoje zasady społeczności.

### Wbudowany prompt początkowy

[inline-code-attrs-start title = 'Początkowy prompt szablonu Moderatora'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Prawie zawsze będziesz chciał **rozszerzyć ten prompt** o konkretne przykłady tego, co Twoja strona akceptuje, a czego nie. Własna polityka eskalacji platformy (ostrzeż przed banem, przeszukaj pamięć przed zablokowaniem) jest już zawarta w promptcie systemowym agenta, więc nie musisz jej powtarzać.

### Wyzwalacze

- **Nowy komentarz dodany** (`COMMENT_ADD`) - agent analizuje każdy nowy komentarz.
- **Komentarz przekroczył próg oznaczeń** (`COMMENT_FLAG_THRESHOLD`, domyślny próg: 3) - agent ponownie ocenia komentarz, który inni użytkownicy oznaczyli.

### Dozwolone narzędzia

- [`mark_comment_approved`](#tools-overview) - przydatne dla tenantów z premoderacją, gdzie agent publikuje czyste komentarze i ukrywa pozostałe.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Nie może publikować komentarzy, głosować, przypinać, blokować, przyznawać odznak ani wysyłać e-maili — prompt jest celowo wąski.

### Zalecane uzupełnienia przed uruchomieniem

- **Ustaw [Zasady społeczności](#community-guidelines).** Kilka zdań pisemnej polityki wystarczy; agent stosuje ją przy każdym uruchomieniu.
- **Zabezpiecz `ban_user` za [zatwierdzeniem](#approval-workflow).** Jest to domyślnie włączone w regionie UE (zob. [Zgodność z art. 17 DSA UE](#eu-dsa-compliance)) i zalecane wszędzie.
- **Rozważ także zabezpieczenie `mark_comment_spam` za zatwierdzeniem**, jeśli masz mały wolumen, ale wysoki wpływ treści.
- **Zabezpiecz `mark_comment_approved` za zatwierdzeniem, jeśli prowadzisz premoderację.** Zatwierdzenie złego komentarza wystawia go przed czytelników; zabezpiecz tę akcję, dopóki agent nie zyska zaufania przez okres testowy.
- **Zaznacz "Uwzględnij współczynnik zaufania komentującego, wiek konta, historię banów i ostatnie komentarze"** w [Opcje kontekstu](#context-options). Model będzie ostrzegać znacznie mniej agresywnie, gdy zobaczy, że ktoś jest długoletnim użytkownikiem działającym w dobrej wierze.

### Zalecany okres testowy

Uruchom ten szablon w [trybie dry-run](#dry-run-mode) przez co najmniej tydzień na realnym ruchu zanim przełączysz go na Włączony. Użyj [Testów (powtórki)](#test-runs-replays), aby również podglądnąć dane z poprzednich 30 dni.

---