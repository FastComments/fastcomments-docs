Уиджетът за масово броене на коментари е проектиран за ефективно показване на броя коментари за множество страници на една и съща страница. Вместо да прави индивидуални API заявки за всеки брой коментари, този уиджет групира заявките за оптимална производителност.

## Основна инсталация

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

## Как работи

Масовият уиджет работи чрез:

1. Сканиране на страницата за елементи с клас `fast-comments-count`
2. Четене на атрибута `data-fast-comments-url-id` от всеки елемент
3. Групиране на API заявки за ефективно извличане на множество бройки коментари
4. Актуализиране на всеки елемент със съответния брой коментари

## Опции за конфигурация

Функцията `FastCommentsCommentCountBulk` приема следните опции за конфигурация:

- **tenantId** (задължително): Вашият FastComments идентификатор на наемател
- **apiHost** (незадължително): Персонализиран API хост, ако използвате самостоятелно хоствана инстанция

## Пример от реалния свят

Ето практически пример, показващ как можете да използвате масовия уиджет в списък с публикации в блог:

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

## Съображения за производителност

Масовият уиджет автоматично оптимизира производителността чрез:

- **Групиране на заявки**: Множество бройки коментари се извличат с една API заявка
- **Ограничения на размера на заявките**: Заявките автоматично се разделят, ако списъкът с URL адреси стане твърде голям (над 1000 символа)
- **Дедупликация**: Множество елементи със същия `data-fast-comments-url-id` споделят същия брой

## Множество елементи със същия URL ID

Можете да имате множество елементи на страницата със същия `data-fast-comments-url-id`. Всички те ще бъдат актуализирани със същия брой:

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

Масовият уиджет автоматично форматира бройките коментари според вашите езикови настройки на FastComments. Той предоставя подходящ текст за:

- Нула коментари
- Един коментар
- Множество коментари

## Кога да използвате масовия срещу единичния уиджет

**Използвайте масовия уиджет когато:**
- Имате множество бройки коментари на една и съща страница
- Показвате списък с публикации/статии с бройки коментари
- Производителността е важна (намалява API заявките)

**Използвайте единичния уиджет когато:**
- Нуждаете се само от един брой коментари на страницата
- Нуждаете се от актуализации на живо (единичният уиджет поддържа актуализации в реално време)
- Искате повече контрол върху поведението на отделния уиджет
