The Recent Comments виджет отображает самые последние комментарии, опубликованные на всём вашем сайте. Он полезен в сайдбарах, нижних колонках или в любом месте, где вы хотите показать свежую активность для стимулирования дальнейшего чтения.

## Options

- **Title** (необязательно): Заголовок, отображаемый над списком. По умолчанию — "Recent Comments".
- **Count** (необязательно): Сколько комментариев показать. Диапазон от 1 до 50. По умолчанию — 5.

## How to Add It

### Inside a Post or Page

В редакторе блоков добавьте блок **Шорткод** и вставьте:

[inline-code-attrs-start title = 'Шорткод Recent Comments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Атрибут `count` принимает любое значение от 1 до 50.

### In a Sidebar or Footer (Classic Themes)

Перейдите в **Внешний вид > Виджеты** в админке WordPress. В вставщике блоков найдите "FastComments" и выберите **FastComments: Recent Comments**. Перетащите его в сайдбар, хедер или футер, затем настройте заголовок и количество через панель виджета.

### In a Block Theme (Full Site Editing)

Откройте **Редактор сайта** в разделе **Внешний вид > Редактор**. Перейдите к той части шаблона, где должен появиться виджет, вставьте блок **Legacy Widget** и выберите из выпадающего списка **FastComments: Recent Comments**.

## Troubleshooting

Виджет отображается только после завершения настройки FastComments и сохранения tenant ID. Если область виджета пустая, завершите настройку в разделе **FastComments** в админке WordPress и перезагрузите страницу.