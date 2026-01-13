Виджет массового подсчета комментариев предназначен для эффективного отображения количества комментариев для нескольких страниц на одной странице. Вместо того чтобы делать отдельные API-запросы для каждого подсчета комментариев, этот виджет группирует запросы для оптимальной производительности.

## Базовая установка

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

## Как это работает

Массовый виджет работает следующим образом:

1. Сканирует страницу на наличие элементов с классом `fast-comments-count`
2. Считывает атрибут `data-fast-comments-url-id` из каждого элемента
3. Группирует API-запросы для эффективного получения нескольких подсчетов комментариев
4. Обновляет каждый элемент соответствующим количеством комментариев

## Параметры конфигурации

Функция `FastCommentsCommentCountBulk` принимает следующие параметры конфигурации:

- **tenantId** (обязательно): Ваш идентификатор арендатора FastComments
- **apiHost** (необязательно): Пользовательский API-хост, если вы используете самостоятельно размещенный экземпляр

## Пример из реальной жизни

Вот практический пример, показывающий, как вы можете использовать массовый виджет в списке постов блога:

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

## Соображения по производительности

Массовый виджет автоматически оптимизирует производительность с помощью:

- **Группировки запросов**: Несколько подсчетов комментариев извлекаются за один API-вызов
- **Ограничения размера запроса**: Запросы автоматически разделяются, если список URL становится слишком большим (более 1000 символов)
- **Дедупликации**: Несколько элементов с одинаковым `data-fast-comments-url-id` используют один и тот же подсчет

## Несколько элементов с одинаковым URL ID

Вы можете иметь несколько элементов на странице с одинаковым `data-fast-comments-url-id`. Все они будут обновлены одинаковым количеством:

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

## Локализация

Массовый виджет автоматически форматирует подсчеты комментариев на основе ваших языковых настроек FastComments. Он предоставляет соответствующий текст для:

- Нуля комментариев
- Одного комментария
- Нескольких комментариев

## Когда использовать массовый виджет против одиночного

**Используйте массовый виджет когда:**
- У вас есть несколько подсчетов комментариев на одной странице
- Вы отображаете список постов/статей с количеством комментариев
- Производительность важна (уменьшает количество API-вызовов)

**Используйте одиночный виджет когда:**
- Вам нужен только один подсчет комментариев на странице
- Вам нужны обновления в реальном времени (одиночный виджет поддерживает обновления в реальном времени)
- Вам нужен больший контроль над поведением отдельного виджета
