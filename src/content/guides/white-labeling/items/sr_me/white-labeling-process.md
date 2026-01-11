---
Поступак за подешавање White Labeling-а је следећи:

1. Изаберите како ће се обрачунавати наплата.
   1. С FastComments Pro, плаћате фиксни износ за до одређеног броја white labeled tenants.
   2. С FastComments Flex, плаћате за сваког tenant-а и за коришћење тог tenant-а.
   3. Ви подешавате лимите за сваког tenant-а у оба случаја.
      1. Лимите је могуће прилагодити по сваком tenant-у. Поред тога, ако ажурирате пакете које продајете, то можете урадити без нарушавања цена које сте већ обезбиједили постојећим купцима.
2. Упознајте се са терминологијом FastComments.com-а:
   1. `Tenant` is a "customer".
   2. `TenantUser` is an administrator with a set of privileges in the `Tenant`.
   3. `TenantPackage` is a package with set of limits and pricing available to a `Tenant`.
3. Интегришите се са нашим [API](/guide-api.html) или користите [скрипте](https://github.com/FastComments/fastcomments-code-examples/tree/master/white-labeling) да онбордујете tenants.

---