Proces konfiguracji White Labeling przedstawia się następująco:

1. Wybierz, w jaki sposób będzie obsługiwane rozliczanie.
   1. W FastComments Pro płacisz stałą kwotę za określoną liczbę tenantów white-label.
   2. W FastComments Flex płacisz za każdego tenanta i za wykorzystanie tego tenanta.
   3. W obu przypadkach ustalasz limity dla każdego tenanta.
      1. Limity mogą być dostosowywane indywidualnie dla każdego tenanta. Dodatkowo, jeśli zaktualizujesz pakiety, które sprzedajesz, możesz to zrobić bez zakłócania cen, które już zaoferowałeś istniejącym klientom.
2. Zapoznaj się z terminologią FastComments.com:
   1. `Tenant` to "klient".
   2. `TenantUser` to administrator z zestawem uprawnień w `Tenant`.
   3. `TenantPackage` to pakiet z zestawem limitów i cen dostępny dla `Tenant`.
3. Zintegruj się z naszym [API](/guide-api.html) lub użyj [skryptów](https://github.com/FastComments/fastcomments-code-examples/tree/master/white-labeling), aby wprowadzić tenantów.