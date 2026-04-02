Po wywołaniu `sdk.load()` SDK automatycznie subskrybuje zdarzenia WebSocket dla skonfigurowanego `urlId`. Obsługiwane są następujące zdarzenia:

- Nowe komentarze, edycje i usunięcia
- Głosy (dodane i usunięte)
- Zmiany stanu przypięcia, blokady, zgłaszania i blokowania
- Obecność użytkowników (wejście/wyjście)
- Otwarcie/zamknięcie wątku
- Przyznawanie odznak
- Aktualizacje konfiguracji serwera

### Sterowanie wyświetlaniem na żywo

Domyślnie nowe komentarze od innych użytkowników pojawiają się natychmiast:

```swift
sdk.showLiveRightAway = true   // Domyślnie: pokaż od razu
```

Ustaw to na `false`, aby buforować nowe komentarze za przyciskiem "N nowych komentarzy", pozwalając użytkownikowi wybrać, kiedy je ujawnić:

```swift
sdk.showLiveRightAway = false
```

### Obecność użytkowników

Wskaźniki online/offline pojawiają się automatycznie na avatarach użytkowników, gdy serwer włącza śledzenie obecności. Po stronie klienta nie jest wymagana dodatkowa konfiguracja.

---
---