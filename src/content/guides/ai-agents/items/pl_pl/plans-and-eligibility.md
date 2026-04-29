AI Agents są dostępni w planach **Flex** i **Pro**. Plan Creator ich nie obejmuje.

### Ograniczenia na poziomie planu

Każdy poziom planu określa:

- **Domyślne dzienne i miesięczne limity budżetu.** Możesz obniżyć je per agent; zwiększenie limitu dla całego konta wymaga planu z wyższym pułapem. Zobacz [Przegląd budżetów](#budgets-overview).

Dokładne wartości są podane na [pricing page](https://fastcomments.com/traffic-pricing) oraz na stronie rozliczeniowej Twojego konta. Są one także wyświetlane bezpośrednio w formularzu edycji agenta, więc nigdy nie musisz opuszczać formularza, aby znaleźć swój limit.

FastComments Pro zawiera $200/mo na wykorzystanie AI. W planie Flex naliczane jest $20 za milion tokenów dla wszystkich modeli (obecnie albo GLM 5.1 albo gpt-oss-120B-turbo).

### Płatności muszą być ważne

AI Agents działają tylko wtedy, gdy tenant ma **ważne dane rozliczeniowe**. Jeśli metoda płatności stanie się nieważna, wszystkie agenty zostają wstrzymane, a strona AI Agents wyświetla baner, który wskazuje, aby osoba z rolą **Billing Admin** zaktualizowała dane rozliczeniowe. Agenty wznowią działanie automatycznie po przywróceniu rozliczeń — nie będzie ponownego uruchamiania ani uzupełniania wyzwalaczy, które wystąpiły podczas przerwy.

To twardy warunek wstępny: wydatki na tokeny są fakturowane na Twoje konto, więc platforma nie wykona żadnego wywołania LLM bez działającej metody płatności.

### Kto może zarządzać agentami

Strony administracyjne agentów są dostępne tylko dla użytkowników z rolą dashboardu **Customization Admin**. **Comment Moderator Admins** mogą przeglądać i decydować o zatwierdzeniach (zobacz [Proces zatwierdzania](#approval-workflow)), ale nie mogą tworzyć ani edytować agentów. **Billing Admins** otrzymują [e-maile z alertami budżetowymi](#budget-alerts) niezależnie od tego, czy mają dostęp do agentów.