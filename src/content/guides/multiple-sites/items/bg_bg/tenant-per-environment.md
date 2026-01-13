Често при FastComments се използва отделен sub tenant за всяка тестова или dev среда. Всеки tenant има собствена конфигурация, данни и API ключове. Конфигурацията, данните и потребителите не могат да се споделят между tenants.
Всичко е изолирано. Въпреки това super admins на parent tenant могат да се представят като потребители в child tenants.

There are two approaches:

- The main tenant is for production, and sub-tenants are for test environments.
- The main tenant is simply for billing, and each sub-tenant is for prod, test, and so on.

The first is generally easier for users to reason about, but this may depend on your organization.

Tenants can be created [тук](https://eu.fastcomments.com/auth/my-account/tenants) if you have the package. Това е и мястото, където super admins биха се представяли като потребители. Tenants могат също да бъдат създадени чрез API за по-персонализирани/автоматизирани настройки.

Какъвто и да е подходът, ще трябва да добавите модераторите и потребителите, които искат да виждат production данните, в "prod" tenant. Така например ако искате
да изберете опция B и да използвате parent tenant за фактуриране, и да имате sub tenant за "prod", ще трябва да добавите tenant-а, да превключите към новия tenant и да добавите вашите
администраторски и модераторски потребители за sub-tenant-а. 

Накрая, за да изясним, страницата за модериране на коментари ще бъде празна при опция B за parent tenant.