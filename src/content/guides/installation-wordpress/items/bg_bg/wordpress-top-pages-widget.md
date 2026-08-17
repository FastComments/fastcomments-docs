The Top Pages widget displays the most-commented pages on your site. It's useful for surfacing your most-engaging content to new visitors and increasing time on site.

## Опции

- **Заглавие** (по избор): The heading shown above the list. Defaults to "Top Pages".

The Top Pages widget chooses its own layout based on available data and does not accept a count attribute.

## Как да го добавите

### В публикация или страница

In the block editor, add a **Shortcode** block and paste:

[inline-code-attrs-start title = 'Кратък код за Top Pages'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### В странична лента или футър (Класически теми)

Go to **Appearance > Widgets** in your WordPress admin. From the block inserter, search for "FastComments" and choose **FastComments: Top Pages**. Drag it into a sidebar, header, or footer area, then set the title from the widget panel.

### В блокова тема (Пълно редактиране на сайта)

Open the **Site Editor** under **Appearance > Editor**. Navigate to the template part where the widget should appear, insert a **Legacy Widget** block, and select **FastComments: Top Pages** from the dropdown.

## Отстраняване на проблеми

The widget only renders after FastComments setup is complete and a tenant ID is stored. If the widget area is blank, complete setup under **FastComments** in the WordPress admin and reload the page.