За повечето сайтове най-лесният начин да добавите коментари е да прикрепите полето `FastComments` към вашите типове съдържание. Отидете в `Structure > Content types > [type] > Manage fields` и добавете полето.

Всеки ентитет, който има полето, получава:

- A **превключвател за статус** така че редакторите да могат да включват или изключват коментирането за всеки ентитет.
- An optional **персонализиран идентификатор**, така че да можете да използвате стабилен ID, който не е обвързан с пътя на Drupal ентитета.

The main `FastComments Widget` block knows about this field, and will skip entities that already have it attached. That way you can mix per-entity comments with the block without seeing the widget twice on the same page.

---