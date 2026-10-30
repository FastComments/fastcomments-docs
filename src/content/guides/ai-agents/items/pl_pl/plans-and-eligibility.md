AI Agents są dostępni w planach **Flex** i **Pro**. Plan Creator ich nie obejmuje.

### Limity na poziomie planu

Każdy poziom planu ustawia:

- **Domyślne dzienne i miesięczne limity budżetu.** Możesz je obniżyć dla poszczególnych agentów; podniesienie limitu na konto wymaga planu z wyższym limitem. Zobacz [Budgets Overview](#budgets-overview).

Dokładne liczby są wyświetlane na [stronie cenowej](https://fastcomments.com/traffic-pricing) oraz na stronie rozliczeń Twojego konta. Są również wyświetlane w formularzu edycji agenta, więc nie musisz opuszczać formularza, aby znaleźć swój limit.

FastComments Pro zawiera $200/miesiąc wartości użycia AI. Flex jest rozliczany w stawce $20 za milion tokenów dla wszystkich modeli (obecnie GLM 5.2 lub gpt-oss-120B).

### Rozliczenia muszą być ważne

Agenci AI działają tylko wtedy, gdy najemca ma **ważne rozliczenia w systemie**. Jeśli metoda płatności stanie się nieważna, wszyscy agenci są wstrzymywani, a strona AI Agents wyświetla baner kierujący osobę z rolą **Billing Admin** do zaktualizowania rozliczeń. Agenci wznawiają działanie samodzielnie po przywróceniu rozliczeń – bez ponownego odtwarzania ani uzupełniania wyzwalaczy, które uruchomiły się podczas awarii.

Jest to twardy warunek wstępny: wydatki na tokeny są fakturowane na Twoje konto, więc platforma nie wyśle żadnego wywołania LLM bez działającej metody płatności.

### Kto może zarządzać agentami

Strony administracyjne agenta są dostępne tylko dla roli **Customization Admin** w panelu. **Comment Moderator Admins** mogą przeglądać i decydować o zatwierdzeniach (zobacz [Approval Workflow](#approval-workflow)), ale nie mogą tworzyć ani edytować agentów. **Billing Admins** otrzymują [e-maile z alertami budżetowymi](#budget-alerts) niezależnie od tego, czy mają dostęp do agenta.