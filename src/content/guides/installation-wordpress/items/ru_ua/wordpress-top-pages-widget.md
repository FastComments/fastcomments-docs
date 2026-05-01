Виджет Топ страниц отображает страницы вашего сайта с наибольшим количеством комментариев. Он полезен для того, чтобы демонстрировать новым посетителям наиболее вовлекающий контент и увеличивать время пребывания на сайте.

## Options

- **Title** (optional): The heading shown above the list. Defaults to "Топ страниц".

Виджет Топ страниц самостоятельно выбирает макет в зависимости от доступных данных и не принимает атрибут count.

## How to Add It

### Inside a Post or Page

In the block editor, add a **Shortcode** block and paste:

[inline-code-attrs-start title = 'Шорткод Топ страниц'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### In a Sidebar or Footer (Classic Themes)

Go to **Appearance > Widgets** in your WordPress admin. From the block inserter, search for "FastComments" and choose **FastComments: Топ страниц**. Drag it into a sidebar, header, or footer area, then set the title from the widget panel.

### In a Block Theme (Full Site Editing)

Open the **Site Editor** under **Appearance > Editor**. Navigate to the template part where the widget should appear, insert a **Legacy Widget** block, and select **FastComments: Топ страниц** from the dropdown.

## Troubleshooting

The widget only renders after FastComments setup is complete and a tenant ID is stored. If the widget area is blank, complete setup under **FastComments** in the WordPress admin and reload the page.