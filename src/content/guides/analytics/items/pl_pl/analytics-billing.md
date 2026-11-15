Wszystkie metryki, dla których możesz podlegać limitom lub rozliczeniom, są raportowane na [stronie Analityki Rozliczeniowej](https://fastcomments.com/auth/my-account/analytics/billing).

Przykładowe śledzone metryki:

- Odsłony stron (w bieżącym miesiącu)
- Kredyty API (w bieżącym miesiącu)
- Nowe zapisane komentarze (w bieżącym miesiącu)
- Użytkownicy tenanta
- Użytkownicy SSO (zwykli użytkownicy SSO bez uprawnień administratora/moderatora)
- Administratorzy SSO (użytkownicy SSO z flagami `isAccountOwner` lub `isAdminAdmin` - rozliczani oddzielnie od zwykłych użytkowników SSO)
- Moderatorzy SSO (użytkownicy SSO z flagą `isCommentModeratorAdmin` - rozliczani oddzielnie od zwykłych użytkowników SSO)
- Moderatorzy
- Domeny

W przypadku planów `Creators` i `Pro` metryki te służą jedynie jako wskazówka, czy zbliżasz się do limitów swojego konta lub je przekroczyłeś.

W przypadku planu `Flex` ta strona pokaże zestawienie kosztów bieżącego użycia dla **następnej** faktury. Dzięki temu łatwo zrozumiesz, jak zostaniesz rozliczony, zanim to nastąpi. Poprzednie faktury możesz zobaczyć w [Historii Rozliczeń](https://fastcomments.com/auth/my-account/billing-history). Sposób liczenia poszczególnych pozycji oraz moment naliczania faktury są opisane w [przewodniku rozliczeniowym](/guide-billing.html#how-flex-usage-is-measured).

Metryki `Page Loads`, `API Credits` i `New Comments Stored` mogą być nieaktualne do jednej minuty. Pozostałe metryki są aktualizowane w czasie rzeczywistym.

Te informacje są przechowywane w ujęciu miesięcznym przez jeden rok, lub do momentu usunięcia swojego tenanta, w celach historycznych, ale interfejs użytkownika pokazuje tylko bieżący okres.