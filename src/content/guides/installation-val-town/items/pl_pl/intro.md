---
[Val Town](https://val.town) uruchamia TypeScript na Deno, więc val jest prawdziwym serwerem. To sprawia, że jest to dobre dopasowanie do FastComments: widget jest tagiem skryptu na stronie, a wszystko, co wymaga sekretu, jak Secure SSO czy weryfikacja webhooka, może działać po stronie serwera w tym samym val.

Ten przewodnik opisuje dodawanie widgetu komentarzy do HTTP val, wyświetlanie liczby komentarzy na stronie indeksu, logowanie użytkowników przy użyciu konta Val Town, które już posiadają, oraz odbieranie webhooków komentarzy.

Nie potrzebujesz konta, aby to wypróbować. Przykłady używają `tenantId: "demo"`, współdzielonego sandboxu, a krok 2 opisuje przejście na własny.

---