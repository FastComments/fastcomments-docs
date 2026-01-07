Віджет масового підрахунку коментарів призначений для ефективного відображення кількості коментарів для кількох сторінок на одній сторінці. Замість того, щоб робити окремі API-запити для кожного підрахунку коментарів, цей віджет групує запити для оптимальної продуктивності.

## Базове встановлення

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Як це працює

Масовий віджет працює наступним чином:

1. Сканує сторінку на наявність елементів з класом `fast-comments-count`
2. Зчитує атрибут `data-fast-comments-url-id` з кожного елемента
3. Групує API-запити для ефективного отримання кількох підрахунків коментарів
4. Оновлює кожен елемент відповідною кількістю коментарів

## Параметри конфігурації

Функція `FastCommentsCommentCountBulk` приймає наступні параметри конфігурації:

- **tenantId** (обов'язково): Ваш ідентифікатор орендаря FastComments
- **apiHost** (необов'язково): Користувацький API-хост, якщо ви використовуєте самостійно розміщений екземпляр

## Приклад з реального життя

Ось практичний приклад, що показує, як ви можете використовувати масовий віджет у списку постів блогу:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Міркування щодо продуктивності

Масовий віджет автоматично оптимізує продуктивність за допомогою:

- **Групування запитів**: Кілька підрахунків коментарів отримуються за один API-виклик
- **Обмеження розміру запиту**: Запити автоматично розділяються, якщо список URL стає занадто великим (понад 1000 символів)
- **Дедуплікації**: Кілька елементів з однаковим `data-fast-comments-url-id` використовують один і той же підрахунок

## Кілька елементів з однаковим URL ID

Ви можете мати кілька елементів на сторінці з однаковим `data-fast-comments-url-id`. Всі вони будуть оновлені однаковою кількістю:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Локалізація

Масовий віджет автоматично форматує підрахунки коментарів на основі ваших мовних налаштувань FastComments. Він надає відповідний текст для:

- Нуля коментарів
- Одного коментаря
- Кількох коментарів

## Коли використовувати масовий віджет проти одиничного

**Використовуйте масовий віджет коли:**
- У вас є кілька підрахунків коментарів на одній сторінці
- Ви відображаєте список постів/статей з кількістю коментарів
- Продуктивність важлива (зменшує кількість API-викликів)

**Використовуйте одиничний віджет коли:**
- Вам потрібен лише один підрахунок коментарів на сторінці
- Вам потрібні оновлення в реальному часі (одиничний віджет підтримує оновлення в реальному часі)
- Вам потрібен більший контроль над поведінкою окремого віджета
