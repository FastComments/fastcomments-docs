Faktura Flex to lista pozycji, po jednej dla każdego rodzaju użycia, z ilością i stawką, na przykład `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Stawki znajdują się na [stronie cennika](https://fastcomments.com/traffic-pricing) oraz na stronie [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Ta sekcja wyjaśnia, co oznacza każda ilość.

### Liczone w ciągu miesiąca

Te liczniki resetują się co miesiąc i są liczone od pierwszego do ostatniego dnia miesiąca kalendarzowego (UTC):

- **Page Loads**: liczba załadowań widżetu komentarzy lub feedu na Twoich stronach. Każde załadowanie widżetu komentarzy się liczy, niezależnie od liczby wyświetlanych komentarzy. Nie ma opłaty za wątek komentarzy ani za stronę.
- **API Credits**: wywołania REST API z Twoim kluczem API.
- **Small Widgets Credits**: załadowania małych widżetów, takich jak liczniki komentarzy i odznaki.
- **LLM Tokens**: tokeny używane przez [AI Agents](/guide-ai-agents.html) i inne funkcje LLM.

### Liczone na koniec miesiąca

To migawka wykonana w momencie tworzenia faktury:

- **Comments Stored**: całkowita liczba komentarzy na Twoim koncie, nie tylko tych dodanych w tym miesiącu. Usuwanie komentarzy ją zmniejsza, z wyjątkiem tego, że usunięty komentarz z odpowiedziami jest zachowywany jako placeholder, aby odpowiedzi nadal miały sens.
- **SSO Users**: całkowita liczba użytkowników SSO utworzonych przez Twoją witrynę, w tym osoby, które komentowały raz dawno temu. Jeśli to największa część Twojego rachunku, zapytaj support o rozliczanie według miesięcznie aktywnych użytkowników, gdzie liczą się tylko użytkownicy, którzy zalogowali się w danym miesiącu.
- **Moderators**, **Admins**, **SSO Moderators** i **SSO Admins**: miejsca (seats), opisane w [Admin and Moderator Seats](#admin-and-moderator-seats). Pierwszy admin jest darmowy.
- **Domains**: witryny na Twoim koncie. Pierwsza witryna jest darmowa, a każda kolejna jest rozliczana miesięcznie, dopóki znajduje się na koncie. Subdomena liczy się tylko wtedy, gdy dodasz ją jako osobną witrynę; pojedyncza domena najwyższego poziomu obejmuje swoje subdomeny. Zobacz [Multiple Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: konta podrzędne zarządzane przez to konto, jeśli istnieją, plus ich własne użycie jako osobne pozycje.

### Minimum

Jeśli pozycje sumują się do mniej niż minimum Flex, faktura jest uzupełniana do minimum. Mała witryna z jednym adminem i jedną domeną płaci tylko minimum.

### Podgląd faktury

Strona [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) pokazuje każdą pozycję następnej faktury z dzisiejszymi liczbami, więc nie ma niespodzianek. Liczniki załadowań stron, API i małych widżetów mogą być opóźnione do minuty; reszta jest na żywo.

### Inne waluty

Użycie Flex jest wyceniane w dolarach amerykańskich. Jeśli waluta Twojej faktury jest inna, każda pozycja jest przeliczana po kursie z dnia utworzenia faktury, więc to samo użycie może kosztować nieco inaczej z miesiąca na miesiąc. Zobacz [Currencies](#currencies).