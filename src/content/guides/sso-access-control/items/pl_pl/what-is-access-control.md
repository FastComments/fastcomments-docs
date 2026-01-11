Dzięki FastComments SSO Access Control, czasami nazywanemu RBAC, można ograniczyć użytkownikom dostęp jedynie do określonych stron lub wątków komentarzy. Dodatkowo użytkownicy mogą `@mention` się nawzajem tylko w tej samej grupie.

## W szczegółach

Możemy umieścić `Users` i opcjonalnie `Pages` w grupach.

Kiedy `Users` są umieszczeni w grupach, a w Ustawieniach Widgetu włączona jest opcja `Limit Comments by SSO User Groups`, użytkownicy będą widzieć tylko komentarze od użytkowników z ich tych samych grup i będą mogli `@mention`ować tylko użytkowników z tych samych grup.

Dodatkowo `Pages` mogą być umieszczone w grupach, dzięki czemu użytkownicy będą mieć dostęp tylko do komentarzy na stronach, do których mają dostęp.

Nazywamy to grupami "na poziomie użytkownika" w przeciwieństwie do grup "na poziomie strony".

Które z nich jest odpowiednie dla Ciebie?

#### Użyj grup na poziomie użytkownika jeśli...

- Chcesz używać tej samej wartości `urlId` (URL strony lub identyfikator artykułu), ale ograniczyć komentarze według grupy.
- Na przykład, chcesz mieć grupy "Nowy użytkownik" i "Doświadczony użytkownik", i nie powinny one nigdy widzieć nawzajem swoich komentarzy na tych samych stronach.

#### Użyj grup na poziomie strony jeśli...

- Twoje grupy zawierają konkretne strony.
- Na przykład, użytkownicy w grupie "Strony publiczne" nigdy nie powinni widzieć artykułów z grupy "Ściśle tajne".

---