Виџет за масовно бројање коментара је дизајниран за ефикасно приказивање броја коментара за више страница на истој страници. Уместо појединачних API позива за сваки број коментара, овај виџет групише захтеве за оптималне перформансе.

## Основна инсталација

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

## Како функционише

Масовни виџет функционише тако што:

1. Скенира страницу за елементе са класом `fast-comments-count`
2. Чита атрибут `data-fast-comments-url-id` из сваког елемента
3. Групише API захтеве за ефикасно преузимање више бројева коментара
4. Ажурира сваки елемент одговарајућим бројем коментара

## Опције конфигурације

Функција `FastCommentsCommentCountBulk` прихвата следеће опције конфигурације:

- **tenantId** (обавезно): Ваш FastComments ID закупца
- **apiHost** (опционо): Прилагођени API хост ако користите сопствену инстанцу

## Пример из стварног света

Ево практичног примера који показује како можете користити масовни виџет у листи блог постова:

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

## Разматрања перформанси

Масовни виџет аутоматски оптимизује перформансе путем:

- **Груписања захтева**: Више бројева коментара се преузима у једном API позиву
- **Ограничења величине захтева**: Захтеви се аутоматски деле ако листа URL-ова постане предугачка (преко 1.000 карактера)
- **Дедупликације**: Више елемената са истим `data-fast-comments-url-id` деле исти број

## Више елемената са истим URL ID

Можете имати више елемената на страници са истим `data-fast-comments-url-id`. Сви ће бити ажурирани истим бројем:

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

## Локализација

Масовни виџет аутоматски форматира бројеве коментара на основу ваших FastComments језичких подешавања. Пружа одговарајући текст за:

- Нула коментара
- Један коментар
- Више коментара

## Када користити масовни наспрам појединачног виџета

**Користите масовни виџет када:**
- Имате више бројева коментара на истој страници
- Приказујете листу постова/чланака са бројевима коментара
- Перформансе су важне (смањује API позиве)

**Користите појединачни виџет када:**
- Потребан вам је само један број коментара на страници
- Потребна су вам ажурирања уживо (појединачни виџет подржава ажурирања у реалном времену)
- Желите више контроле над понашањем појединачног виџета
