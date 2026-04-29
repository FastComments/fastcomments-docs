---
**Template ID:** `tos_enforcer`

Szablon Moderator jest zalecanym punktem wyjścia, jeśli Twoim celem jest zmniejszenie obciążenia moderacji ręcznej. Przegląda nowe i oznaczone komentarze oraz stosuje zasady Twojej społeczności.

Zazwyczaj będziesz chciał **rozszerzyć wbudowany prompt** o konkretne przykłady tego, co na Twojej stronie jest dozwolone, a co nie. Własna polityka eskalacji platformy (ostrzeżenie przed banem, przeszukanie pamięci przed zbanowaniem) jest już wbudowana w prompt systemowy, który agent otrzymuje, więc nie musisz jej powtarzać.

### Triggers

- **New comment posted** (`COMMENT_ADD`) - agent przegląda każdy nowy komentarz.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - agent ponownie ocenia komentarz, który został oznaczony przez innych użytkowników.

### Allowed tools

- [`mark_comment_approved`](#tools-overview) - przydatne w środowiskach z przedmoderacją, gdzie agent publikuje zatwierdzone komentarze i ukrywa pozostałe.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Nie może publikować komentarzy, głosować, przypinać, blokować, przyznawać odznak ani wysyłać e-maili — prompt jest celowo ograniczony.

### Recommended additions before going live

- **Set [Community Guidelines](#community-guidelines).** Kilka zdań pisemnej polityki wystarczy; agent stosuje je przy każdym uruchomieniu.
- **Gate `ban_user` behind [approval](#approval-workflow).** Jest to domyślnie włączone w regionie UE (zobacz [EU DSA Article 17 Compliance](#eu-dsa-compliance)) i zalecane wszędzie.
- **Consider also gating `mark_comment_spam` behind approval** jeśli masz mały wolumen, ale treści o dużych konsekwencjach.
- **Gate `mark_comment_approved` behind approval if you run pre-moderation.** Zatwierdzenie złego komentarza wystawia go przed czytelników; zabezpiecz tę akcję, dopóki agent nie zyska zaufania poprzez tryb próbny.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** in [Context Options](#context-options). Model będzie ostrzegać znacznie mniej agresywnie, gdy będzie mógł zobaczyć, że ktoś jest długoletnim użytkownikiem działającym w dobrej wierze.

### Recommended dry-run window

Uruchom ten szablon w [dry-run](#dry-run-mode) przez co najmniej tydzień na rzeczywistym ruchu przed przełączeniem na Enabled. Użyj [Test Runs (Replays)](#test-runs-replays), aby również podejrzeć wyniki za ostatnie 30 dni.

---