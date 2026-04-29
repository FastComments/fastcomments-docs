Wyzwalane, gdy komentarz zostanie automatycznie oznaczony jako spam przez wbudowany mechanizm antyspamowy FastComments — **nie** przez moderatora i nie przez innego agenta.

### Kontekst otrzymywany przez agenta

- Komentarz oznaczony automatycznie jako spam.
- Opcjonalna historia wątku / historii użytkownika / kontekst strony zgodnie z konfiguracją.

### Kto wyzwala to

Potok antyspamowy platformy. Zobacz [Wykrywanie spamu](/guide-moderation.html#spam-detection) w przewodniku po moderacji, aby uzyskać więcej informacji.

### Typowe zastosowania

- **Moderacja z drugiego spojrzenia** - silnik antyspamowy ma wysoką czułość (recall), ale niedoskonałą precyzję; agent wytrenowany na specyficznym stylu Twojej społeczności może wychwycić fałszywe pozytywy. Agent może wywołać funkcję, aby cofnąć oznaczenie błędnie sklasyfikowanego komentarza jako spam.
- **Automatyczne odblokowywanie** - jeśli Twój tenant agresywnie blokuje nowe konta z powodu spamu, agent reagujący na to wyzwalanie może przejrzeć i usunąć oczywiste fałszywe pozytywy zanim ktokolwiek je zobaczy.

### Ważne

- Wyzwalacz **nie** uruchamia się dla spamu oznaczonego przez moderatora (użyj [Trigger: Moderator Marked Spam](#trigger-moderator-spammed)) ani dla spamu oznaczonego przez innego agenta.
- Komentarz, który został automatycznie oznaczony jako spam, a następnie oznaczony przez moderatora jako Not Spam, nie spowoduje ponownego uruchomienia wyzwalacza.
- Subskrypcja tego wyzwalacza jest najbardziej przydatna w tenantach, w których tryb automatycznego oznaczania spamu silnika jest włączony w Ustawieniach moderacji. W przeciwnym razie wyzwalacz nie zostanie uruchomiony.