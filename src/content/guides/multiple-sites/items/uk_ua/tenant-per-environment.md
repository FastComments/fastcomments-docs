Зазвичай у FastComments для кожного test або dev середовища створюють окремий дочірній тенант. Кожний тенант матиме власну конфігурацію, дані, та API keys. Конфігурація, дані та користувачі не можуть бути спільними між тенантами.
Усе ізольовано. Однак суперадміністратори батьківського тенанта можуть виконувати дії від імені користувачів у дочірніх тенантах.

There are two approaches:

- The main tenant is for production, and sub-tenants are for test environments.
- The main tenant is simply for billing, and each sub-tenant is for prod, test, and so on.

Перший варіант загалом простіший для розуміння користувачами, але це може залежати від вашої організації.

Тенанти можна створити [тут](https://eu.fastcomments.com/auth/my-account/tenants), якщо у вас є пакет. Це також місце, де суперадміністратори
від імені користувачів можуть увійти. Тенанти також можна створити через API для більш індивідуальних/автоматизованих налаштувань.

Який би підхід ви не обрали, вам доведеться додати модераторів і користувачів, які хочуть бачити produktion-дані в тенанті "prod". So for example if you want
to go with option B and have the parent tenant for billing, and have a sub tenant for "prod", you'll want to add the tenant, switch to the new tenant, and add your
admin and moderator users for the sub-tenant. 

Finally, to clarify, the Moderate Comments page will be empty with option B for the parent tenant.