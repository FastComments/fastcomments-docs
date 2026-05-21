### Zainstaluj ze Sklepu z aplikacjami Shopify

1. Otwórz [wpis FastComments w sklepie z aplikacjami Shopify](https://apps.shopify.com/fastcomments).
2. Kliknij **Add app** i wybierz plan podczas procesu instalacji.
3. Po zakończeniu instalacji Shopify przekieruje Cię z powrotem do panelu administracyjnego FastComments wewnątrz Shopify.

To cała instalacja. Nie musisz niczego wklejać do plików motywu.

### Co zostanie dla Ciebie skonfigurowane

Proces instalacji wykonuje wszystko, co normalnie trzeba by zrobić ręcznie:

- Dla Twojego sklepu zostaje utworzony tenant FastComments i powiązany z domeną sklepu.
- URL sklepu zostaje dodany do autoryzowanych domen tenant-a, dzięki czemu komentarze ładują się bez błędu domeny.
- Zostaje zapisane pole metafield sklepu `fastcomments.tenant_id`, dzięki czemu każdy blok wie, który tenant ma renderować.
- Single sign-on dla klientów Shopify jest domyślnie włączony.
- Rozliczenia odbywają się przez Shopify Managed Pricing. Opłaty pojawiają się na Twoim standardowym rachunku Shopify. Możesz ulepszać, obniżać lub anulować subskrypcję w **Ustawienia > Aplikacje i kanały sprzedaży > FastComments** w panelu administracyjnym Shopify.

Jeżeli Twój sklep był już wcześniej klientem FastComments przed instalacją aplikacji, instalator użyje istniejącego tenant-a zamiast tworzyć nowego.

### Zintegrowany panel administracyjny

Po otwarciu aplikacji FastComments z panelu administracyjnego Shopify trafisz na pulpit z kafelkami, które jednym kliknięciem przenoszą do pełnego zaplecza FastComments:

- **Dashboard**: ustawienia konta, statystyki użycia i szczegóły subskrypcji.
- **Moderation Queue**: zatwierdzanie, odrzucanie i odpowiadanie na komentarze w całym sklepie.
- **Customize**: dostosowywanie kolorów widgetu, czcionek, reguł moderacji i konfiguracji.
- **Ratings & Reviews Helper**: konfiguracja ocen gwiazdkowych i pytań recenzji, jeśli chcesz użyć bloku Reviews Summary.

Każdy kafelek otwiera FastComments za pomocą jednorazowego linku logowania, więc nie potrzebujesz oddzielnego logowania.

### Następny krok: dodaj bloki do swojego sklepu

Otwórz edytor motywu Shopify (**Online Store > Themes > Customize**), otwórz szablon, do którego chcesz dodać komentarze lub recenzje, i kliknij **Add block**. Bloki FastComments pojawią się pod **Apps**. Reszta tego przewodnika opisuje każdy z nich.