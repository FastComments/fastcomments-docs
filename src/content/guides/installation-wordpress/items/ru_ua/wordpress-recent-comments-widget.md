---
Виджет Recent Comments отображает самые последние комментарии, опубликованные по всему вашему сайту. Он полезен в боковых панелях, футерах или в любом месте, где вы хотите показать свежую активность, чтобы побудить к дальнейшему чтению.

## Options

- **Title** (optional): The heading shown above the list. Defaults to "Recent Comments".
- **Count** (optional): How many comments to show. Range 1 to 50. Defaults to 5.

## How to Add It

### Inside a Post or Page

In the block editor, add a **Shortcode** block and paste:

[inline-code-attrs-start title = 'Шорткод Recent Comments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### In a Sidebar or Footer (Classic Themes)

Go to **Appearance > Widgets** in your WordPress admin. From the block inserter, search for "FastComments" and choose **FastComments: Recent Comments**. Drag it into a sidebar, header, or footer area, then configure the title and count from the widget panel.

### In a Block Theme (Full Site Editing)

Open the **Site Editor** under **Appearance > Editor**. Navigate to the template part where the widget should appear, insert a **Legacy Widget** block, and select **FastComments: Recent Comments** from the dropdown.

## Troubleshooting

The widget only renders after FastComments setup is complete and a tenant ID is stored. If the widget area is blank, complete setup under **FastComments** in the WordPress admin and reload the page.

---