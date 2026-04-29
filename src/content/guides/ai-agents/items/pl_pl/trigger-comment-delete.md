Wywoływane, gdy komentarz zostaje usunięty.

### Kontekst, który otrzymuje agent

- Komentarz, który właśnie został usunięty - tekst, autor, strona.
- Opcjonalny kontekst wątku / historii użytkownika / strony zgodnie z konfiguracją.

### Warto wiedzieć

- Wywoływane zarówno dla **soft deletes** (gdy komentarz jest ukryty, ale zachowany do celów audytu), jak i dla **hard deletes** (gdy komentarz zostaje całkowicie usunięty). Obsługa wyzwalacza rozwiązuje komentarz z potoku kaskadowego usuwania; to, co widzi agent, to ostatni znany stan.
- Gdy komentarz zostanie w pełni usunięty, narzędzia, które celują w niego (`pin_comment`, `mark_comment_spam`, itd.) dla tego ID komentarza nie będą działać.

### Typowe zastosowania

- **Przekazywanie audytu przez [Webhooki](#webhooks-overview)** - wysyłaj zdarzenie `trigger.succeeded`, aby zewnętrzny system zarejestrował, co zostało usunięte.
- **Zapisy w pamięci** - spraw, by agent zapisał [notatkę pamięci](#tools-overview) o wzorcu usunięć (usunięty komentarz był trzecim komentarzem użytkownika w ciągu 24 godzin, itd.).
- **Efekty między wątkami** - zauważ, gdy usunięcie zmienia strukturę wątku, który agent wcześniej podsumował, i rozważ, czy należy ponownie podsumować.

### Uwaga dotycząca kosztów operacyjnych

Jeśli masz serwis o dużym wolumenie usunięć (intensywna moderacja ręczna), ten wyzwalacz może być wywoływany często.