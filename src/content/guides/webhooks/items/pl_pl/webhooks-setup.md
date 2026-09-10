---
Postępuj zgodnie z tymi samymi krokami dla `localhost`, tak jak w środowisku produkcyjnym. Upewnij się, że masz skonfigurowane domeny produkcyjne i sekrety API.

Najpierw przejdź do [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Jest to dostępne poprzez Zarządzanie danymi -> Webhooks.

Strona wyświetla wszystkie webhooki w Twoim koncie:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Strona administracji webhookami wyświetlająca każdy webhook wraz z jego URL, zdarzeniem, domeną, metodą, statusem i liczbą oczekujących zdarzeń'; title='Lista webhooków'; cacheBuster = 'v4' app-screenshot-end]

Kliknij **Nowy webhook**, aby dodać go. Każdy webhook ma URL, jedno zdarzenie komentarza (utworzone, zaktualizowane lub usunięte), domenę oraz metodę HTTP:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Formularz nowego webhooka z polami URL, zdarzenie, domena i metoda HTTP oraz przyciskiem Wyślij testowy ładunek'; title='Nowy webhook'; cacheBuster = 'v4' app-screenshot-end]

Każdy webhook jest dostarczany niezależnie. Możesz wysłać to samo zdarzenie do kilku punktów końcowych, a webhook o zakresie **Wszystkie domeny** otrzymuje komentarze ze wszystkich domen, nawet gdy istnieje webhook specyficzny dla domeny dla tego samego zdarzenia. Ten sam URL, zdarzenie i domena nie mogą być dodane dwukrotnie.

Przed zapisaniem kliknij **Wyślij testowy ładunek**, aby sprawdzić, czy punkt końcowy akceptuje podpisane żądanie. Zobacz następną sekcję, „Testowanie”, aby uzyskać szczegóły.

Z listy możesz edytować, wyłączyć, ponownie włączyć lub usunąć webhook. Wyłączenie zachowuje oczekujące zdarzenia, aż webhook zostanie ponownie włączony; usunięcie powoduje ich odrzucenie.

Webhooki mogą być również tworzone za pośrednictwem API, na przykład przez Zapier. Pojawiają się na tej samej liście ze źródłem **API**. Zobacz Zarządzanie webhookami za pomocą API.

---