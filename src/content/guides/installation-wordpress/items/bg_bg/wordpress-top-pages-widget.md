The Top Pages widget displays the most-commented pages on your site. It's useful for surfacing your most-engaging content to new visitors and increasing time on site.

## Options

- **Title** (optional): The heading shown above the list. Defaults to "Top Pages".

The Top Pages widget chooses its own layout based on available data and does not accept a count attribute.

## How to Add It

### Inside a Post or Page

In the block editor, add a **Shortcode** block and paste:

[inline-code-attrs-start title = 'Шорткод за Top Pages'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### In a Sidebar or Footer (Classic Themes)

Go to **Appearance > Widgets** in your WordPress admin. From the block inserter, search for "FastComments" and choose **FastComments: Top Pages**. Drag it into a sidebar, header, or footer area, then set the title from the widget panel.

### In a Block Theme (Full Site Editing)

Open the **Site Editor** under **Appearance > Editor**. Navigate to the template part where the widget should appear, insert a **Legacy Widget** block, and select **FastComments: Top Pages** from the dropdown.

## Troubleshooting

The widget only renders after FastComments setup is complete and a tenant ID is stored. If the widget area is blank, complete setup under **FastComments** in the WordPress admin and reload the page.