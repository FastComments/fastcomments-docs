Wywoływane, gdy moderator oznaczy komentarz jako przejrzany.

### Kontekst, który otrzymuje agent

- Komentarz.
- **ID użytkownika wyzwalającego** - moderator, który przejrzał.
- Opcjonalna historia wątku / użytkownika / kontekst strony zgodnie z konfiguracją.

### Kto wyzwala to zdarzenie

Ręczna akcja moderatora na stronie moderacji, w widżecie komentarzy lub za pośrednictwem API.

### Typowe zastosowania

- **Przekazywanie audytu** za pomocą [Webhooks](#webhooks-overview).
- **Zapis w pamięci** - zanotuj, że ten komentarz został przejrzany przez człowieka, aby inni agenci nie przetwarzali go ponownie.

### Warto zauważyć

- "Przejrzany" jest jednym ze stanów kolejki moderacji śledzonym oddzielnie od "zatwierdzony" i "spam". Komentarz może być zatwierdzony-i-przejrzany, zatwierdzony-ale-nieprzejrzany, itd. Zobacz [Jak działają zatwierdzenia](/guide-moderation.html#moderation-approvals) w przewodniku moderacji.
- To wyzwalacz o dużej częstotliwości działania w tenantach z wieloma moderatorami. Subskrybuj selektywnie i odpowiednio zaplanuj budżet.

---