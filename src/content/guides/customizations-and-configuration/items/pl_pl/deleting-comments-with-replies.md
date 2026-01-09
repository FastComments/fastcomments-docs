Domyślnie użytkownicy mogą usuwać własne komentarze. Ponadto usunięcie ich komentarza automatycznie usuwa wszystkie komentarze podrzędne i tymczasowe w wątku. To zachowanie jest również aktywne na żywo.

Możesz ograniczyć to w następujący sposób:

- Zamiast tego zanonimizuj usunięty komentarz (ustaw name i text na `[deleted]` lub wartość niestandardową).
- Nie pozwalaj na usuwanie komentarzy, gdy istnieją odpowiedzi. Wyświetlany jest konfigurowalny komunikat o błędzie.
- Ogranicz możliwość usuwania komentarzy z odpowiedziami tylko do administratorów i moderatorów.

To można skonfigurować w sekcji `Comment Thread Deletion` w interfejsie dostosowywania widżetu.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]

---