The **FastComments - Reviews Summary** block shows an aggregated star rating and review breakdown for a page. Pair it with the **FastComments** block on product templates for the standard reviews layout: summary up top, review form and reviews below.

### Передумова: налаштуйте Ratings & Reviews

Блок Reviews Summary відображає питання рейтингу, які ви налаштували для вашого магазину. Налаштуйте їх спочатку:

1. Відкрийте додаток FastComments у вашій адмінці Shopify.
2. Клацніть плитку **Ratings & Reviews Helper** (або відкрийте [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) безпосередньо).
3. Додайте питання, на які має відповісти кожен рецензент (загальний рейтинг зірками, «як підійшло», тощо).

Без налаштованих питань блоку підсумку нічого агрегувати.

### Додати блок

1. Відкрийте редактор теми Shopify.
2. Відкрийте шаблон **Product** (або шаблон сторінки, де ви хочете розмістити підсумок).
3. Натисніть **Add block** близько до верхньої частини секції сторінки, над місцем, де буде блок **FastComments**.
4. У розділі **Apps** виберіть **FastComments - Reviews Summary**.
5. Додайте блок **FastComments** нижче на тій самій сторінці, якщо ви ще цього не зробили, щоб відвідувачі могли залишати відгуки.
6. Натисніть **Save**.

### Налаштування

| Налаштування | Що робить | За замовчуванням |
|---|---|---|
| Tenant ID (optional) | Override which FastComments tenant the summary reads from. Leave blank to use the store's automatically-configured tenant. | (blank) |
| Custom URL ID | Override the page identifier the summary aggregates against. Use this when the summary lives on a different page from the FastComments block it reflects. | (auto-detected) |

### Як підсумок зіставляється з відгуками

Блок Reviews Summary використовує ту ж логіку автоматичного визначення, що й блок **FastComments**:

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

Для звичайної сторінки продукту підсумок і тред коментарів автоматично мають спільний URL ID, без необхідності додаткової конфігурації.

### Поради

- Підсумок лише для читання. Щоб збирати відгуки, потрібен блок **FastComments** на тій самій сторінці.
- Якщо ви зміните питання оцінювання в Ratings & Reviews Helper після збору відгуків, підсумок буде перераховано відповідно до нового набору питань.